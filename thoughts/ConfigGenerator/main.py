
import os

def read_file(name):
    with open(f'res/{name}', 'r') as f:
        return f.read()


def write_file(name, content):
    with open(f'C:/_C_/Home/Produce/Code/Projects/Rust/BigYoshis/LiteralPoggySource/.idea/runConfigurations/{name}', 'w') as f: f.write(content)


if __name__ == '__main__':
    run_configs_dir = "C:/_C_/Home/Produce/Code/Projects/Rust/BigYoshis/LiteralPoggySource/.idea/runConfigurations"
    if not os.path.exists(run_configs_dir):
        os.makedirs(run_configs_dir)
    game = "bibble"
    names = ["Atoms", "QuickToast", "Oberdiah", "Legend", "Numcake", "Shotekri", "ConnorHS", "Lain", "Guest1", "Guest2", "Guest3"]
    server = read_file("ServerTemplate.xml")

    module_name = game.lower()
    server_name = "S"
    server_file_name = server_name.replace(" ", "_")
    write_file(server_file_name + ".xml", server.replace("[[1]]", server_name)
               .replace("[[2]]", module_name))

    client = read_file("ClientTemplate.xml")
    compound = read_file("CompoundTemplate.xml")
    for i in range(1, 7):
        client_name = "C" + str(i)
        write_file(client_name + ".xml", client.replace("[[1]]", client_name)
                   .replace("[[2]]", module_name)
                   .replace("[[3]]", names[i - 1]))

        compound_name = "S C" + str(i)
        compound_file_name = compound_name.replace(" ", "_")
        projects = f"<toRun name=\"{server_name}\" type=\"CargoCommandRunConfiguration\" />"
        for b in range(1, i + 1):
            my_game_name = "C" + str(b)
            projects += f"<toRun name=\"{my_game_name}\" type=\"CargoCommandRunConfiguration\" />"
        write_file(compound_file_name + ".xml", compound.replace("[[1]]", compound_name)
                   .replace("[[2]]", projects))








