from src.nodes import *
from src.tokens_and_keywords import *
from src.context import Context
from src.symbol_table import SymbolTable
from src.interpreter_and_values import Number, Interpreter
from src.errors import IllegalCharError, InvalidSyntaxError, ExpectedCharacterError


class Token:
    def __init__(self, type, value=None, pos_start=None, pos_end=None):
        self.type = type
        self.value = value

        if pos_start:
            self.pos_start = pos_start.copy()
            self.pos_end = pos_start.copy()
            self.pos_end.advance()

        if pos_end:
            self.pos_end = pos_end

    def matches(self, type, value):
        return self.type == type and self.value == value

    def __repr__(self):
        if self.value:
            return f'{self.type}:{self.value}'

        return f'{self.type}'


class Position:
    def __init__(self, index: int, line_num: int, column_num: int, file_name: str, file_text: str):
        self.index = index
        self.line_num = line_num
        self.column_num = column_num
        self.file_name = file_name
        self.file_text = file_text

    def advance(self, current_char=None):
        self.index += 1
        self.column_num += 1

        if current_char == '\n':
            self.line_num += 1
            self.column_num = 0

        return self

    def copy(self):
        return Position(self.index, self.line_num, self.column_num, self.file_name, self.file_text)


class Lexer:
    def __init__(self, file_name: str, text: str):
        self.file_name = file_name
        self.text = text
        self.pos = Position(-1, 0, -1, file_name, text)
        self.current_char = None

        self.advance()

    def advance(self):
        self.pos.advance(self.current_char)
        self.current_char = self.text[self.pos.index] if self.pos.index < len(self.text) else None

    def makeTokens(self) -> tuple[list, None]:
        tokens = []

        while self.current_char is not None:
            if self.current_char in ' \t':
                self.advance()

            elif self.current_char in DIGITS:
                tokens.append(self.makeNumber())

            elif self.current_char in LETTERS + '_':
                tokens.append(self.makeIdentifier())

            elif self.current_char == '+':
                tokens.append(Token(TT_PLUS, pos_start=self.pos))
                self.advance()

            elif self.current_char == '-':
                tokens.append(self.makeMinusOrArrow())

            elif self.current_char == '*':
                tokens.append(Token(TT_MUL, pos_start=self.pos))
                self.advance()

            elif self.current_char == '/':
                tokens.append(Token(TT_DIV, pos_start=self.pos))
                self.advance()

            elif self.current_char == '^':
                tokens.append(Token(TT_POW, pos_start=self.pos))
                self.advance()

            elif self.current_char == '(':
                tokens.append(Token(TT_LPAREN, pos_start=self.pos))
                self.advance()

            elif self.current_char == ')':
                tokens.append(Token(TT_RPAREN, pos_start=self.pos))
                self.advance()

            elif self.current_char == '!':
                tok, error = self.makeNotEquals()

                if error:
                    return [], error

                tokens.append(tok)

            elif self.current_char == '=':
                tokens.append(self.makeEquals())

            elif self.current_char == '<':
                tokens.append(self.makeLessThan())

            elif self.current_char == '>':
                tokens.append(self.makeGreaterThan())

            elif self.current_char == ',':
                tokens.append(Token(TT_COMMA, pos_start=self.pos))
                self.advance()

            else:
                pos_start = self.pos.copy()
                char = self.current_char
                self.advance()
                return [], IllegalCharError(pos_start, self.pos, f'"{char}" is not defined')

        tokens.append(Token(TT_EOF, pos_start=self.pos))
        return tokens, None

    def makeNumber(self) -> Token:
        num_str = ''
        dot_count = 0
        pos_start = self.pos.copy()

        while self.current_char is not None and self.current_char in DIGITS + '.':
            if self.current_char == '.':
                if dot_count == 1:
                    break

                dot_count += 1
                num_str += '.'

            else:
                num_str += self.current_char

            self.advance()

        if dot_count == 0:
            return Token(TT_INT, int(num_str), pos_start, self.pos)

        else:
            return Token(TT_FLOAT, float(num_str), pos_start, self.pos)

    def makeIdentifier(self) -> Token:
        id_str = ''
        pos_start = self.pos.copy()

        while self.current_char is not None and self.current_char in LETTERS_DIGITS + '_':
            id_str += self.current_char
            self.advance()

        token_type = TT_KEYWORD if id_str in KEYWORDS else TT_IDENTIFIER

        return Token(token_type, id_str, pos_start, self.pos)

    def makeMinusOrArrow(self) -> Token:
        token_type = TT_MINUS
        pos_start = self.pos.copy()
        self.advance()

        if self.current_char == '>':
            self.advance()
            token_type = TT_ARROW

        return Token(token_type, pos_start=pos_start, pos_end=self.pos)

    def makeEquals(self) -> Token:
        token_type = TT_EQ
        pos_start = self.pos.copy()
        self.advance()

        if self.current_char == '=':
            self.advance()
            token_type = TT_EE

        return Token(token_type, pos_start=pos_start, pos_end=self.pos)

    def makeNotEquals(self):
        pos_start = self.pos.copy()
        self.advance()

        if self.current_char == '=':
            self.advance()
            return Token(TT_NE, pos_start=pos_start, pos_end=self.pos), None

        self.advance()
        return None, ExpectedCharacterError(pos_start, self.pos, '"=" (after "!")')

    def makeLessThan(self) -> Token:
        token_type = TT_LT
        pos_start = self.pos.copy()
        self.advance()

        if self.current_char == '=':
            self.advance()
            token_type = TT_LTE

        return Token(token_type, pos_start=pos_start, pos_end=self.pos)

    def makeGreaterThan(self) -> Token:
        token_type = TT_GT
        pos_start = self.pos.copy()
        self.advance()

        if self.current_char == '=':
            self.advance()
            token_type = TT_GTE

        return Token(token_type, pos_start=pos_start, pos_end=self.pos)


class ParseResult:
    def __init__(self):
        self.error = None
        self.node = None
        self.advance_count = 0

    def registerAdvancement(self):
        self.advance_count += 1

    def register(self, res):
        self.advance_count += res.advance_count

        if res.error:
            self.error = res.error

        return res.node

    def success(self, node):
        self.node = node
        return self

    def failure(self, error):
        if not self.error or self.advance_count == 0:
            self.error = error
        return self


class Parser:
    def __init__(self, tokens: list[Token]):
        self.tokens = tokens
        self.token_index = -1

        self.advance()

    def advance(self):
        self.token_index += 1

        if self.token_index < len(self.tokens):
            self.current_token = self.tokens[self.token_index]

        return self.current_token

    def parse(self):
        result = self.expr()

        if not result.error and self.current_token.type is not TT_EOF:
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected operators "+", "-", "*" or "/"'
            ))

        return result

    def ifExpr(self):
        result = ParseResult()
        cases = []
        else_case = None

        if not self.current_token.matches(TT_KEYWORD, 'if'):
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "if"'
            ))

        result.registerAdvancement()
        self.advance()

        condition = result.register(self.expr())

        if result.error:
            return result

        if not self.current_token.matches(TT_KEYWORD, 'then'):
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "then"'
            ))

        result.registerAdvancement()
        self.advance()

        expr = result.register(self.expr())

        if result.error:
            return result

        cases.append((condition, expr))

        while self.current_token.matches(TT_KEYWORD, 'alsoif'):
            result.registerAdvancement()
            self.advance()

            condition = result.register(self.expr())

            if result.error:
                return result

            if not self.current_token.matches(TT_KEYWORD, 'then'):
                return result.failure(InvalidSyntaxError(
                    self.current_token.pos_start, self.current_token.pos_end,
                    'Expected "then"'
                ))

            result.registerAdvancement()
            self.advance()

            expr = result.register(self.expr())

            if result.error:
                return result

            cases.append((condition, expr))

        if self.current_token.matches(TT_KEYWORD, 'otherwise'):
            result.registerAdvancement()
            self.advance()

            else_case = result.register(self.expr())

            if result.error:
                return result

        return result.success(IfNode(cases, else_case))

    def forExpr(self):
        result = ParseResult()

        if not self.current_token.matches(TT_KEYWORD, 'walk'):
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "walk"'
            ))

        result.registerAdvancement()
        self.advance()

        if self.current_token.type != TT_IDENTIFIER:
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected identifier'
            ))

        var_name = self.current_token
        result.registerAdvancement()
        self.advance()

        if self.current_token.type != TT_EQ:
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "="'
            ))

        result.registerAdvancement()
        self.advance()

        start_value = result.register(self.expr())

        if result.error:
            return result

        if not self.current_token.matches(TT_KEYWORD, 'through'):
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "through"'
            ))

        result.registerAdvancement()
        self.advance()

        end_value = result.register(self.expr())

        if result.error:
            return result

        if self.current_token.matches(TT_KEYWORD, 'step'):
            result.registerAdvancement()
            self.advance()

            step_value = result.register(self.expr())

            if result.error:
                return result

        else:
            step_value = None

        if not self.current_token.matches(TT_KEYWORD, 'then'):
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "then"'
            ))

        result.registerAdvancement()
        self.advance()

        body = result.register(self.expr())

        if result.error:
            return result

        return result.success(ForNode(var_name, start_value, end_value, step_value, body))

    def whileExpr(self):
        result = ParseResult()

        if not self.current_token.matches(TT_KEYWORD, 'while'):
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "while"'
            ))

        result.registerAdvancement()
        self.advance()

        condition = result.register(self.expr())

        if result.error:
            return result

        if not self.current_token.matches(TT_KEYWORD, 'then'):
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "then"'
            ))

        result.registerAdvancement()
        self.advance()

        body = result.register(self.expr())

        if result.error:
            return result

        return result.success(WhileNode(condition, body))

    def call(self):
        result = ParseResult()
        atom = result.register(self.atom())

        if result.error:
            return result

        if self.current_token.type == TT_LPAREN:
            result.registerAdvancement()
            self.advance()

            arg_nodes = []

            if self.current_token.type == TT_RPAREN:
                result.registerAdvancement()
                self.advance()

            else:
                arg_nodes.append(result.register(self.expr()))

                if result.error:
                    return result.failure(InvalidSyntaxError(
                        self.current_token.pos_start, self.current_token.pos_end,
                        'Expected ")", "smt", "if", "walk", "while", "func", int, float, identifier, "+", "-", '
                        '"(" or "oppositeof"'
                    ))

                while self.current_token.type == TT_COMMA:
                    result.registerAdvancement()
                    self.advance()

                    arg_nodes.append(result.register(self.expr()))

                    if result.error:
                        return result

                if self.current_token.type != TT_RPAREN:
                    return result.failure(InvalidSyntaxError(
                        self.current_token.pos_start, self.current_token.pos_end,
                        'Expected "," or ")"'
                    ))

                result.registerAdvancement()
                self.advance()

            return result.success(CallNode(atom, arg_nodes))

        return result.success(atom)

    def atom(self):
        result = ParseResult()
        token = self.current_token

        if token.type in (TT_INT, TT_FLOAT):
            result.registerAdvancement()
            self.advance()
            return result.success(NumberNode(token))

        elif token.type == TT_IDENTIFIER:
            result.registerAdvancement()
            self.advance()
            return result.success(VariableAccessNode(token))

        elif token.type in (TT_LPAREN, TT_RPAREN):
            result.registerAdvancement()
            self.advance()
            expr = result.register(self.expr())

            if result.error:
                return result

            if self.current_token.type == TT_RPAREN:
                result.registerAdvancement()
                self.advance()
                return result.success(expr)

            else:
                return result.failure(InvalidSyntaxError(
                    self.current_token.pos_start, self.current_token.pos_end,
                    'Expected ")"'
                ))

        elif token.matches(TT_KEYWORD, 'if'):
            if_expr = result.register(self.ifExpr())

            if result.error:
                return result

            return result.success(if_expr)

        elif token.matches(TT_KEYWORD, 'walk'):
            for_expr = result.register(self.forExpr())

            if result.error:
                return result

            return result.success(for_expr)

        elif token.matches(TT_KEYWORD, 'while'):
            while_expr = result.register(self.whileExpr())

            if result.error:
                return result

            return result.success(while_expr)

        elif token.matches(TT_KEYWORD, 'func'):
            func_def = result.register(self.functionDefinition())

            if result.error:
                return result

            return result.success(func_def)

        return result.failure(InvalidSyntaxError(
            token.pos_start, token.pos_end, 'Expected "smt", "if", "walk", "while", "func", int, float, identifier, '
                                            '"+", "-" or "("'
        ))

    def power(self):
        return self.binaryOperator(self.call, (TT_POW,), self.factor)

    def factor(self) -> ParseResult:
        result = ParseResult()
        token = self.current_token

        if token.type in (TT_PLUS, TT_MINUS):
            result.registerAdvancement()
            self.advance()
            factor = result.register(self.factor())

            if result.error:
                return result

            return result.success(UnaryOperatorNode(token, factor))

        return self.power()

    def term(self):
        return self.binaryOperator(self.factor, (TT_MUL, TT_DIV))

    def arithmeticExpr(self):
        return self.binaryOperator(self.term, (TT_PLUS, TT_MINUS))

    def comparisonExpr(self):
        result = ParseResult()

        if self.current_token.matches(TT_KEYWORD, 'oppositeof'):
            op_token = self.current_token
            result.registerAdvancement()
            self.advance()

            node = result.register(self.comparisonExpr())

            if result.error:
                return result

            return result.success(UnaryOperatorNode(op_token, node))

        node = result.register(self.binaryOperator(self.arithmeticExpr, (TT_EE, TT_NE, TT_LT, TT_GT, TT_LTE, TT_GTE)))

        if result.error:
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected int, float, identifier, "+", "-", "(" or "NOT"'

            ))

        return result.success(node)

    def expr(self):
        result = ParseResult()

        if self.current_token.matches(TT_KEYWORD, 'smt'):
            result.registerAdvancement()
            self.advance()

            if self.current_token.type != TT_IDENTIFIER:
                return result.failure(InvalidSyntaxError(
                    self.current_token.pos_start, self.current_token.pos_end,
                    'Expected identifier'
                ))

            var_name = self.current_token
            result.registerAdvancement()
            self.advance()

            if self.current_token.type != TT_EQ:
                return result.failure(InvalidSyntaxError(
                    self.current_token.pos_start, self.current_token.pos_end,
                    'Expected "="'
                ))

            result.registerAdvancement()
            self.advance()
            expr = result.register(self.expr())

            if result.error:
                return result

            return result.success(VariableAssignNode(var_name, expr))

        node = result.register(self.binaryOperator(self.comparisonExpr, ((TT_KEYWORD, 'and'), (TT_KEYWORD, 'or'))))

        if result.error:
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "smt", "if", "walk", "while", "func", int, float, identifier, "+", "-", "(" or "oppositeof"'
            ))

        return result.success(node)

    def functionDefinition(self):
        result = ParseResult()

        if not self.current_token.matches(TT_KEYWORD, 'func'):
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                f"Expected 'FUN'"
            ))

        result.registerAdvancement()
        self.advance()

        if self.current_token.type == TT_IDENTIFIER:
            var_name_token = self.current_token
            result.registerAdvancement()
            self.advance()

            if self.current_token.type != TT_LPAREN:
                return result.failure(InvalidSyntaxError(
                    self.current_token.pos_start, self.current_token.pos_end,
                    'Expected "("'
                ))

        else:
            var_name_token = None
            if self.current_token.type != TT_LPAREN:
                return result.failure(InvalidSyntaxError(
                    self.current_token.pos_start, self.current_token.pos_end,
                    'Expected identifier or "("'
                ))

        result.registerAdvancement()
        self.advance()

        arg_name_tokens = []

        if self.current_token.type == TT_IDENTIFIER:
            arg_name_tokens.append(self.current_token)

            result.registerAdvancement()
            self.advance()

            while self.current_token.type == TT_COMMA:
                result.registerAdvancement()
                self.advance()

                if self.current_token.type != TT_IDENTIFIER:
                    return result.failure(InvalidSyntaxError(
                        self.current_token.pos_start, self.current_token.pos_end,
                        'Expected Identifier'
                    ))

                arg_name_tokens.append(self.current_token)

                result.registerAdvancement()
                self.advance()

            if self.current_token.type != TT_RPAREN:
                return result.failure(InvalidSyntaxError(
                    self.current_token.pos_start, self.current_token.pos_end,
                    'Expected "," or ")"'
                ))

        else:
            if self.current_token.type != TT_RPAREN:
                return result.failure(InvalidSyntaxError(
                    self.current_token.pos_start, self.current_token.pos_end,
                    'Expected identifier or ")"'
                ))

        result.registerAdvancement()
        self.advance()

        if self.current_token.type != TT_ARROW:
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "->"'
            ))

        result.registerAdvancement()
        self.advance()

        node_to_return = result.register(self.expr())

        if result.error:
            return result

        return result.success(FunctionDefinitionNode(
            var_name_token,
            arg_name_tokens,
            node_to_return
        ))

    def binaryOperator(self, func_a, ops, func_b=None):
        if func_b is None:
            func_b = func_a

        result = ParseResult()
        left = result.register(func_a())
        if result.error:
            return result

        while self.current_token.type in ops or (self.current_token.type, self.current_token.value) in ops:
            op_token = self.current_token
            result.registerAdvancement()
            self.advance()
            right = result.register(func_b())

            if result.error:
                return result

            left = BinaryOperatorNode(left, op_token, right)

        return result.success(left)


global_symbol_table = SymbolTable()
global_symbol_table.set('nothing', Number(0))
global_symbol_table.set('true', Number(1))
global_symbol_table.set('false', Number(0))


def run(file_name: str, text: str):
    lexer = Lexer(file_name, text)
    tokens, error = lexer.makeTokens()

    if error:
        return None, error

    parser = Parser(tokens)
    ast = parser.parse()

    if ast.error:
        return None, ast.error

    interpreter = Interpreter()
    context = Context('<program>')
    context.symbol_table = global_symbol_table
    result = interpreter.visit(ast.node, context)

    return result.value, result.error
