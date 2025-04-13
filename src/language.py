from src.lexer import Lexer
from src.parser import Parser
from src.context import Context
from src.interpreter_and_values import Interpreter


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
    context.symbol_table = interpreter.global_symbol_table
    result = interpreter.visit(ast.node, context)

    return result.value, result.error
