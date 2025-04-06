from src.nodes import NumberNode, BinaryOperatorNode, UnaryOperatorNode, VariableAssignNode, VariableAccessNode
from src.symbol_table import SymbolTable
from src.values import Number
from src.errors import IllegalCharError, InvalidSyntaxError, RunTimeError
import string

DIGITS = '0123456789'
LETTERS = string.ascii_letters
LETTERS_DIGITS = LETTERS + DIGITS
TT_INT = 'INT'
TT_FLOAT = 'FLOAT'
TT_IDENTIFIER = 'IDENTIFIER'
TT_KEYWORD = 'KEYWORD'
TT_PLUS = 'PLUS'
TT_MINUS = 'MINUS'
TT_MUL = 'MUL'
TT_DIV = 'DIV'
TT_POW = 'POW'
TT_EQ = 'EQ'
TT_LPAREN = 'LPAREN'
TT_RPAREN = 'RPAREN'
TT_EOF = 'EOF'
KEYWORDS = [
    'smt'
]


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
                tokens.append(Token(TT_MINUS, pos_start=self.pos))
                self.advance()

            elif self.current_char == '*':
                tokens.append(Token(TT_MUL, pos_start=self.pos))
                self.advance()

            elif self.current_char == '/':
                tokens.append(Token(TT_DIV, pos_start=self.pos))
                self.advance()

            elif self.current_char == '^':
                tokens.append(Token(TT_POW, pos_start=self.pos))
                self.advance()

            elif self.current_char == '=':
                tokens.append(Token(TT_EQ, pos_start=self.pos))
                self.advance()

            elif self.current_char == '(':
                tokens.append(Token(TT_LPAREN, pos_start=self.pos))
                self.advance()

            elif self.current_char == ')':
                tokens.append(Token(TT_RPAREN, pos_start=self.pos))
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

        return result.failure(InvalidSyntaxError(
            token.pos_start, token.pos_end, 'Expected "smt", int, float, identifier, "+", "-" or "("'
        ))

    def power(self):
        return self.binaryOperator(self.atom, (TT_POW), self.factor)

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

        node = result.register(self.binaryOperator(self.term, (TT_PLUS, TT_MINUS)))

        if result.error:
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "smt", int, float, identifier, "+", "-" or "("'
            ))

        return result.success(node)

    def binaryOperator(self, func_a, ops, func_b=None):
        if func_b is None:
            func_b = func_a

        result = ParseResult()
        left = result.register(func_a())
        if result.error:
            return result

        while self.current_token.type in ops:
            op_token = self.current_token
            result.registerAdvancement()
            self.advance()
            right = result.register(func_b())

            if result.error:
                return result

            left = BinaryOperatorNode(left, op_token, right)

        return result.success(left)


class Interpreter:
    def visit(self, node, context):
        method_name = f'visit_{type(node).__name__}'
        method = getattr(self, method_name, self.noVisitMethod)

        return method(node, context)

    def noVisitMethod(self, node, context):
        raise Exception(f'No visit_{type(node).__name__} method defined')

    def visit_NumberNode(self, node: NumberNode, context):
        return RuntimeResult().success(
            Number(node.token.value).setContext(context).setPos(node.pos_start, node.pos_end))

    def visit_VariableAccessNode(self, node: VariableAccessNode, context):
        result = RuntimeResult()
        var_name = node.var_name_tokem.value
        value = context.symbol_table.get(var_name)

        if not value:
            return result.failure(
                RunTimeError(node.pos_start,
                             node.pos_end,
                             f'"{var_name}" is not defined',
                             context))

        value = value.copy().setPos(node.pos_start, node.pos_end)
        return result.success(value)

    def visit_VariableAssignNode(self, node: VariableAssignNode, context):
        result = RuntimeResult()
        var_name = node.var_name_token.value
        value = result.register(self.visit(node.value_node, context))

        if result.error:
            return result

        context.symbol_table.set(var_name, value)
        return result.success(value)

    def visit_BinaryOperatorNode(self, node: BinaryOperatorNode, context):
        result = RuntimeResult()
        left = result.register(self.visit(node.left_node, context))

        if result.error:
            return result

        right = result.register(self.visit(node.right_node, context))

        if result.error:
            return result

        if node.op_token.type == TT_PLUS:
            number, error = left.addedTo(right)
        elif node.op_token.type == TT_MINUS:
            number, error = left.subtractedBy(right)
        elif node.op_token.type == TT_MUL:
            number, error = left.multipliedBy(right)
        elif node.op_token.type == TT_DIV:
            number, error = left.dividedBy(right)
        elif node.op_token.type == TT_POW:
            number, error = left.poweredBy(right)

        if error:
            return result.failure(error)

        else:
            return result.success(number.setPos(node.pos_start, node.pos_end))

    def visit_UnaryOperatorNode(self, node: UnaryOperatorNode, context):
        result = RuntimeResult()
        number = result.register(self.visit(node.node, context))

        if result.error:
            return result

        error = None

        if node.op_token.type == TT_MINUS:
            number, error = number.multipliedBy(Number(-1))

        if error:
            return result.failure(error)

        else:
            return result.success(number.setPos(node.pos_start, node.pos_end))


class RuntimeResult:
    def __init__(self):
        self.value = None
        self.error = None

    def register(self, result):
        if result.error:
            self.error = result.error

        return result.value

    def success(self, value):
        self.value = value
        return self

    def failure(self, error):
        self.error = error
        return self


class Context:
    def __init__(self, display_name, parent=None, parent_entry_pos=None):
        self.display_name = display_name
        self.parent = parent
        self.parent_entry_pos = parent_entry_pos
        self.symbol_table = None


global_symbol_table = SymbolTable()
global_symbol_table.set('nothing', Number(0))


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
