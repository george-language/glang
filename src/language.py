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


class Lexer:
    def __init__(self, text: str):
        self.text = text
        self.pos = -1
        self.current_char = None

        self.advance()

    def advance(self):
        self.pos += 1
        self.current_char = self.text[self.pos] if self.pos < len(self.text) else None

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
                char = self.current_char
                self.advance()
                return [], IllegalCharError(f'"{char}" is not defined')

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


def run(text: str) -> tuple[list, None]:
    lexer = Lexer(text)
    tokens, error = lexer.makeTokens()

    return tokens, error