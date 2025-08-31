json_file(
    name = "json_file",
    file = "data.json",
    outputs = {
        "list": "names",
    }
)

template(
    name = "formater",
    pattern = "Bonjour {name}",
    data = {
        "name": "Alexandre"
    }
)

output(
    name = "file_gen",
    file = "output.md",
    content = ":formater",
)

run(
    name = "default",
    target = ":file_gen",
)