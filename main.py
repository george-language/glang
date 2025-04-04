import sys
from src.language import run
from src.errors import WrongFileTypeError


file_path = sys.argv[1]

if file_path.endswith('.glang'):
    with open(file_path, 'r') as f:
        text = f.read()

        result, error = run(text)

        if error:
            print(error.asString())

        else:
            print(result)

else:
    print(WrongFileTypeError(f'"{file_path}" is not a ".glang" file.').asString())