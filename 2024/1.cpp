#include <bits/stdc++.h>
#include "readfile.hpp"


int main(int argc, char const *argv[])
{
	int res = 0;
	std::vector<std::string> file = readfile("1.in");
	if (file.empty()) {
		return 1;
	}
	std::vector<int> left, right;
	for (const auto &line : file) {
		int n1, n2;
		// put to vector
		std::istringstream iss(line);
        if (!(iss >> n1 >> n2)) {
            std::cerr << "Invalid input line: " << line << std::endl;
            return 1;
        }
        left.push_back(n1);
        right.push_back(n2);
	}

	// sort
    std::sort(left.begin(), left.end());
    std::sort(right.begin(), right.end());

	if (left.size() != right.size()) {
		std::cerr << "size mismatch: " << left.size() << " vs " << right.size() << std::endl;
		return 1;
	}

	// calc total distant
	for (int i = 0; i < left.size(); i++) {
		res += std::abs(left[i] - right[i]);
	}

	std::cout << "Result: " << res << std::endl;
	return 0;
}
