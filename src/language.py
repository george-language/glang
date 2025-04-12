from src.token_types_and_keywords import *
from src.token import Token
from src.parser import Parser
from src.context import Context
from src.symbol_table import SymbolTable
from src.interpreter_and_values import Number, Interpreter, BuiltInFunction
from src.errors import IllegalCharError, ExpectedCharacterError


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

            elif self.current_char == '"':
                tokens.append(self.makeString())

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

            elif self.current_char == '[':
                tokens.append(Token(TT_LSQUARE, pos_start=self.pos))
                self.advance()

            elif self.current_char == ']':
                tokens.append(Token(TT_RSQUARE, pos_start=self.pos))
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

    def makeString(self) -> Token:
        string = ''
        pos_start = self.pos.copy()
        escape_char = False
        self.advance()

        escape_chars = {
            'n': '\n',
            't': '\t',
        }

        while self.current_char is not None and (self.current_char != '"' or escape_char):
            if escape_char:
                string += escape_chars.get(self.current_char, self.current_char)

            else:
                if self.current_char == '\\':
                    escape_char = True

                else:
                    string += self.current_char

            self.advance()
            escape_char = False

        self.advance()
        return Token(TT_STR, string, pos_start=pos_start, pos_end=self.pos)

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


Number.null = Number(0)
Number.true = Number(1)
Number.false = Number(0)
BuiltInFunction.print = BuiltInFunction('print')
BuiltInFunction.input = BuiltInFunction('input')
BuiltInFunction.isnumber = BuiltInFunction('isnumber')
BuiltInFunction.isstring = BuiltInFunction('isstring')
BuiltInFunction.islist = BuiltInFunction('islist')
BuiltInFunction.isfunction = BuiltInFunction('isfunction')
BuiltInFunction.append = BuiltInFunction('append')
BuiltInFunction.pop = BuiltInFunction('pop')
BuiltInFunction.extend = BuiltInFunction('extend')
BuiltInFunction.reverse = BuiltInFunction('reverse')
BuiltInFunction.reversed = BuiltInFunction('reversed')
BuiltInFunction.clear = BuiltInFunction('clear')
BuiltInFunction.length = BuiltInFunction('length')

global_symbol_table = SymbolTable()
global_symbol_table.set('emptybowl', Number.null)
global_symbol_table.set('true', Number.true)
global_symbol_table.set('false', Number.false)
global_symbol_table.set('bark', BuiltInFunction.print)
global_symbol_table.set('chew', BuiltInFunction.print)
global_symbol_table.set('isnumber', BuiltInFunction.isnumber)
global_symbol_table.set('isstring', BuiltInFunction.isstring)
global_symbol_table.set('islist', BuiltInFunction.islist)
global_symbol_table.set('isfunction', BuiltInFunction.isfunction)
global_symbol_table.set('append', BuiltInFunction.append)
global_symbol_table.set('pop', BuiltInFunction.pop)
global_symbol_table.set('extend', BuiltInFunction.extend)
global_symbol_table.set('reverse', BuiltInFunction.reverse)
global_symbol_table.set('reversed', BuiltInFunction.reversed)
global_symbol_table.set('clear', BuiltInFunction.clear)
global_symbol_table.set('lengthof', BuiltInFunction.length)


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
