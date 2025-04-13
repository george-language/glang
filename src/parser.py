from src.token_types_and_keywords import *
from src.nodes import *
from src.token import Token
from src.errors import InvalidSyntaxError


class ParseResult:
    def __init__(self):
        self.error = None
        self.node = None
        self.last_registered_advance_count = 0
        self.advance_count = 0
        self.to_reverse_count = 0

    def registerAdvancement(self):
        self.last_registered_advance_count = 1
        self.advance_count += 1

    def register(self, res):
        self.last_registered_advance_count = res.advance_count
        self.advance_count += res.advance_count

        if res.error:
            self.error = res.error

        return res.node

    def tryRegister(self, res):
        if res.error:
            self.to_reverse_count = res.advance_count

            return None

        return self.register(res)

    def success(self, node):
        self.node = node
        return self

    def failure(self, error):
        if not self.error or self.last_registered_advance_count == 0:
            self.error = error
        return self


class Parser:
    def __init__(self, tokens: list[Token]):
        self.tokens = tokens
        self.token_index = -1

        self.advance()

    def advance(self):
        self.token_index += 1
        self.updateCurrentToken()
        return self.current_token

    def reverse(self, amount=1):
        self.token_index -= amount
        self.updateCurrentToken()
        return self.current_token

    def updateCurrentToken(self):
        if self.token_index >= 0 and self.token_index < len(self.tokens):
            self.current_token = self.tokens[self.token_index]

    def parse(self):
        result = self.statements()

        if not result.error and self.current_token.type is not TT_EOF:
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected operators "+", "-", "*", "/" or "endbody"'
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
                    'Expected "]", "object", "if", "walk", "while", "func", int, float, identifier, "+", "-", "(", "[" or "oppositeof"'
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
        all_cases = result.register(self.ifExprCases('if'))

        if result.error:
            return result

        cases, else_case = all_cases
        return result.success(IfNode(cases, else_case))

    def ifExpr_b(self):
        return self.ifExprCases('alsoif')

    def ifExpr_c(self):
        result = ParseResult()
        else_case = None

        if self.current_token.matches(TT_KEYWORD, 'otherwise'):
            result.registerAdvancement()
            self.advance()

            if self.current_token.type == TT_NEWLINE:
                result.registerAdvancement()
                self.advance()

                statements = result.register(self.statements())

                if result.error:
                    return result

                else_case = (statements, True)

                if self.current_token.matches(TT_KEYWORD, 'endbody'):
                    result.registerAdvancement()
                    self.advance()

                else:
                    return result.failure(InvalidSyntaxError(
                        self.current_token.pos_start, self.current_token.pos_end,
                        'Expected "endbody"'
                    ))

            else:
                expr = result.register(self.statement())

                if result.error:
                    return result

                else_case = (expr, False)

        return result.success(else_case)

    def ifExpr_b_or_c(self):
        result = ParseResult()
        cases, else_case = [], None

        if self.current_token.matches(TT_KEYWORD, 'alsoif'):
            all_cases = result.register(self.ifExpr_b())

            if result.error:
                return result

            cases, else_case = all_cases

        else:
            else_case = result.register(self.ifExpr_c())

            if result.error:
                return result

        return result.success((cases, else_case))

    def ifExprCases(self, case_keyword):
        result = ParseResult()
        cases = []
        else_case = None

        if not self.current_token.matches(TT_KEYWORD, case_keyword):
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                f'Expected "{case_keyword}"'
            ))

        result.registerAdvancement()
        self.advance()

        condition = result.register(self.statement())

        if result.error:
            return result

        if not self.current_token.matches(TT_KEYWORD, 'then'):
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "then"'
            ))

        result.registerAdvancement()
        self.advance()

        if self.current_token.type == TT_NEWLINE:
            result.registerAdvancement()
            self.advance()

            statements = result.register(self.statements())

            if result.error:
                return result

            cases.append((condition, statements, True))

            if self.current_token.matches(TT_KEYWORD, 'endbody'):
                result.registerAdvancement()
                self.advance()

            else:
                all_cases = result.register(self.ifExpr_b_or_c())

                if result.error:
                    return result

                new_cases, else_case = all_cases
                cases.extend(new_cases)

        else:
            expr = result.register(self.expr())

            if result.error:
                return result

            cases.append((condition, expr, False))

            all_cases = result.register(self.ifExpr_b_or_c())

            if result.error:
                return result

            new_cases, else_case = all_cases
            cases.extend(new_cases)

        return result.success((cases, else_case))

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

        if self.current_token.type == TT_NEWLINE:
            result.registerAdvancement()
            self.advance()

            body = result.register(self.statements())

            if result.error:
                return result

            if not self.current_token.matches(TT_KEYWORD, 'endbody'):
                return result.failure(InvalidSyntaxError(
                    self.current_token.pos_start, self.advance().pos_end,
                    'Expected "endbody"'
                ))

            result.registerAdvancement()
            self.advance()

            return result.success(ForNode(var_name, start_value, end_value, step_value, body, True))

        body = result.register(self.statement())

        if result.error:
            return result

        return result.success(ForNode(var_name, start_value, end_value, step_value, body, False))

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

        if self.current_token.type == TT_NEWLINE:
            result.registerAdvancement()
            self.advance()

            body = result.register(self.statements())

            if result.error:
                return result

            if not self.current_token.matches(TT_KEYWORD, 'endbody'):
                return result.failure(InvalidSyntaxError(
                    self.current_token.pos_start, self.current_token.pos_end,
                    'Expected "endbody"'
                ))

            result.registerAdvancement()
            self.advance()

            return result.success(WhileNode(condition, body, True))

        body = result.register(self.statement())

        if result.error:
            return result

        return result.success(WhileNode(condition, body, False))

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

        if self.current_token.matches(TT_KEYWORD, 'object'):
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
                'Expected "object", "if", "walk", "while", "func", int, float, identifier, "+", "-", "(", "[" or "oppositeof"'
            ))

        return result.success(node)

    def statement(self):
        result = ParseResult()
        pos_start = self.current_token.pos_start.copy()

        if self.current_token.matches(TT_KEYWORD, 'give'):
            result.registerAdvancement()
            self.advance()

            expr = result.tryRegister(self.expr())

            if not expr:
                self.reverse(result.to_reverse_count)

            return result.success(ReturnNode(expr, pos_start=pos_start, pos_end=self.current_token.pos_start.copy()))

        if self.current_token.matches(TT_KEYWORD, 'next'):
            result.registerAdvancement()
            self.advance()

            return result.success(ContinueNode(pos_start=pos_start, pos_end=self.current_token.pos_start.copy()))

        if self.current_token.matches(TT_KEYWORD, 'leave'):
            result.registerAdvancement()
            self.advance()

            return result.success(BreakNode(pos_start=pos_start, pos_end=self.current_token.pos_start.copy()))

        expr = result.register(self.expr())

        if result.error:
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "give", "next", "leave", "object", "if", "walk", "while", "func", int, float, identifier, '
                '"+", "-", "(", "[" or "oppositeof"'
            ))

        return result.success(expr)

    def statements(self):
        result = ParseResult()
        statements = []
        pos_start = self.current_token.pos_start.copy()

        while self.current_token.type == TT_NEWLINE:
            result.registerAdvancement()
            self.advance()

        statement = result.register(self.statement())

        if result.error:
            return result

        statements.append(statement)

        more_statements = True

        while True:
            newline_count = 0

            while self.current_token.type == TT_NEWLINE:
                result.registerAdvancement()
                self.advance()

                newline_count += 1

            if newline_count == 0:
                more_statements = False

            if not more_statements:
                break

            statement = result.tryRegister(self.statement())

            if not statement:
                self.reverse(result.to_reverse_count)
                more_statements = False

                continue

            statements.append(statement)

        return result.success(ListNode(
            statements,
            pos_start,
            self.current_token.pos_end.copy()
        ))

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
                        'Expected ")", "object", "if", "walk", "while", "func", int, float, identifier, "+", "-", '
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
            token.pos_start, token.pos_end, 'Expected "object", "if", "walk", "while", "func", int, float, identifier, '
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
                'Expected "func"'
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

        if self.current_token.type == TT_ARROW:
            result.registerAdvancement()
            self.advance()

            node_to_return = result.register(self.expr())

            if result.error:
                return result

            return result.success(FunctionDefinitionNode(
                var_name_token,
                arg_name_tokens,
                node_to_return,
                True
            ))

        if self.current_token.type != TT_NEWLINE:
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "->" or newline'
            ))

        result.registerAdvancement()
        self.advance()

        body = result.register(self.statements())

        if result.error:
            return result

        if not self.current_token.matches(TT_KEYWORD, 'endbody'):
            return result.failure(InvalidSyntaxError(
                self.current_token.pos_start, self.current_token.pos_end,
                'Expected "endbody"'
            ))

        result.registerAdvancement()
        self.advance()

        return result.success(FunctionDefinitionNode(
            var_name_token,
            arg_name_tokens,
            body,
            False
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
