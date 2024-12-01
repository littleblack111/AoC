#include <iostream>
#include <fstream>
#include <vector>
#include "readfile.hpp"

std::vector<std::string> readfile(const std::string &name) {
    std::vector<std::string> file;
	std::ifstream f(name);

    if (!f) {
        std::cerr << "Error: Unable to open file" << std::endl;
        return {};
    }

	std::string line;
	while (getline(f, line)) {
		file.push_back(line);
	}

	f.close();
	return file;
}
