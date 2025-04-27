import sys
import os.path
from src.language import run

VERSION = '1.2'

command = None

try:
    command = sys.argv[1]

except:
    pass

if command:
    if command.endswith('.glang'):
        if os.path.exists(command):
            with open(command, 'r') as f:
                text = f.read()

                result, error = run(command, text)

                if error:
                    print(error.asString())

        else:
            print(f'File "{command}" is not a valid file')

    elif command == 'new':
        project_name = None

        try:
            project_name = sys.argv[2]

        except:
            print('Usage: "new" [project_name]')

        if project_name:
            os.mkdir(project_name)
            os.mkdir(f'{project_name}/src')

            with open(f'{project_name}/main.glang', 'w') as f:
                f.write('func main()\n\tbark("Hello, world!")\nendbody\n\nmain()')

    elif command == 'init':
        os.mkdir(f'src')

        with open(f'main.glang', 'w') as f:
            f.write('func main()\n\tbark("Hello, world!")\nendbody\n\nmain()')

    else:
        print(f'Argument Undefined: "{command}" is not a recognized as any GLang command.')

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
