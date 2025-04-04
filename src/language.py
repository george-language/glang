from src.errors import IllegalCharError


DIGITS = '0123456789'
TT_INT = 'INT'
TT_FLOAT = 'FLOAT'
TT_PLUS = 'PLUS'
TT_MINUS = 'MINUS'
TT_MUL = 'MUL'
TT_DIV = 'DIV'
TT_LPAREN = 'LPAREN'
TT_RPAREN = 'RPAREN'


class Token:
    def __init__(self, type, value=None):
        self.type = type
        self.value = value

    def __repr__(self):
        if self.value: return f'{self.type}:{self.value}'
        return f'{self.type}'


class Position:
    def __init__(self, index: int, line_num: int, column_num: int, file_name: str, file_text: str):
        self.index = index
        self.line_num = line_num
        self.column_num = column_num
        self.file_name = file_name
        self.file_text = file_text

    def advance(self, current_char):
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

            elif self.current_char == '+':
                tokens.append(Token(TT_PLUS))
                self.advance()

            elif self.current_char == '-':
                tokens.append(Token(TT_MINUS))
                self.advance()

            elif self.current_char == '*':
                tokens.append(Token(TT_MUL))
                self.advance()

            elif self.current_char == '/':
                tokens.append(Token(TT_DIV))
                self.advance()

            elif self.current_char == '(':
                tokens.append(Token(TT_LPAREN))
                self.advance()

            elif self.current_char == ')':
                tokens.append(Token(TT_RPAREN))
                self.advance()

            else:
                pos_start = self.pos.copy()
                char = self.current_char
                self.advance()
                return [], IllegalCharError(pos_start, self.pos, f'"{char}" is not defined')

        return tokens, None

    def makeNumber(self) -> Token:
        num_str = ''
        dot_count = 0

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
            return Token(TT_INT, int(num_str))

        else:
            return Token(TT_FLOAT, float(num_str))


def run(file_name: str, text: str) -> tuple[list, None]:
    lexer = Lexer(file_name, text)
    tokens, error = lexer.makeTokens()

    return tokens, error