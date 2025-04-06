from src.language import run

while True:
    text = input('GeorgeLanguage >>> ')

    if text == 'exit()':
        break

    result, error = run('<stdin>', text)

    if error:
        print(error.asString())

    else:
        print(result)
