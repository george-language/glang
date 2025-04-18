import sys
import os.path
from src.language import run

VERSION = '1.1'

file_path = None

try:
    file_path = sys.argv[1]

except:
    pass

if file_path:
    if file_path.endswith('.glang'):
        if os.path.exists(file_path):
            with open(file_path, 'r') as f:
                text = f.read()

                result, error = run(file_path, text)

                if error:
                    print(error.asString())

                elif result:
                    print(f'George Debug: {repr(result)}')

        else:
            print(f'File "{file_path}" is not a valid file')

    else:
        print(f'Wrong File Type: "{file_path}" is not a ".glang" file.')

else:
    print(f'George Language {VERSION} Platform {sys.platform}')
    print('Type "exit()" to exit')

    while True:
        text = input('>>> ')

        if text == 'exit()':
            break
        elif text.strip() == '':
            continue

        result, error = run('<stdin>', text)

        if error:
            print(error.asString())

        elif result:
            print(f'George Debug: {repr(result)}')
