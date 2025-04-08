from src.language import run
import sys


VERSION = '1.0.0'

print(f'George Language {VERSION} Platform {sys.platform}')
print('Type "exit()" to exit')

while True:
    text = input('>>> ')

    if text == 'exit()':
        break

    result, error = run('<stdin>', text)

    if error:
        print(error.asString())

    elif result:
        print(result)
