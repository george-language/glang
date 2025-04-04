import sys
import os.path
from src.language import run


file_path = sys.argv[1]

if file_path.endswith('.glang') and os.path.exists(file_path):
    with open(file_path, 'r') as f:
        text = f.read()

        result, error = run(file_path, text)

        if error:
            print(error.asString())

        else:
            print(result)

else:
    print(f'Wrong File Type: "{file_path}" is not a ".glang" file.')