from src.token_types_and_keywords import *
from src.nodes import *
from src.token import Token
from src.errors import InvalidSyntaxError


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

    def listExpr(self):
        result = ParseResult()
        element_nodes = []
        pos_start = self.current_token.pos_start.copy()

        if self.current_token.type != TT_LSQUARE:
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "["'
            ))

        result.registerAdvancement()
        self.advance()

        if self.current_token.type == TT_RSQUARE:
            result.registerAdvancement()
            self.advance()

        else:
            element_nodes.append(result.register(self.expr()))

            if result.error:
                return result.failure(InvalidSyntaxError(
                    self.current_token.pos_start, self.current_token.pos_end,
                    'Expected "]", "smt", "if", "walk", "while", "func", int, float, identifier, "+", "-", "(", "[" or "oppositeof"'
                ))

            while self.current_token.type == TT_COMMA:
                result.registerAdvancement()
                self.advance()

                element_nodes.append(result.register(self.expr()))

                if result.error:
                    return result

            if self.current_token.type != TT_RSQUARE:
                return result.failure(InvalidSyntaxError(
                    self.current_token.pos_start, self.current_token.pos_end,
                    'Expected "," or "]"'
                ))

            result.registerAdvancement()
            self.advance()

        return result.success(ListNode(
            element_nodes,
            pos_start,
            self.current_token.pos_end.copy()
        ))

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
                'Expected int, float, identifier, "+", "-", "(", "[" or "oppositeof"'

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
                'Expected "smt", "if", "walk", "while", "func", int, float, identifier, "+", "-", "(", "[" or "oppositeof"'
            ))

        return result.success(node)

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
                        '"(", "[" or "oppositeof"'
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

        if token.type == TT_STR:
            result.registerAdvancement()
            self.advance()

            return result.success(StringNode(token))

        elif token.type == TT_IDENTIFIER:
            result.registerAdvancement()
            self.advance()
            return result.success(VariableAccessNode(token))

        elif token.type == TT_LPAREN:
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

        elif token.type == TT_LSQUARE:
            list_expr = result.register(self.listExpr())

            if result.error:
                return result

            return result.success(list_expr)

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
                                            '"+", "-" "(" or "["'
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
