#include <bits/stdc++.h>
#include "readfile.hpp"

int get_problem(const std::vector<int> &numbers) {
	bool increasing = false, decreasing = false;
	int prev = -1;
	for (int i = 0; i < numbers.size(); ++i) {
		if (prev == -1) {
			prev = numbers[i];
			continue;
		}
		if (prev < numbers[i])
			increasing = true;
		else if (prev > numbers[i])
			decreasing = true;
		if (std::abs(prev - numbers[i]) > 3 || prev == numbers[i] || increasing && decreasing || !increasing && !decreasing)
			return i;
		prev = numbers[i];
	}
	return -1;
}


int main(int argc, char const *argv[])
{
	int res = 0;
	std::vector<std::string> file = readfile("2.in");
	if (file.empty())
		return 1;

	for (const auto &line : file) {
		std::istringstream iss(line);
		std::vector<int> lv;
		int n;
		while (iss >> n) {
			lv.push_back(n);
		}
		if (get_problem(lv) == -1) {
			++res;
			continue;
		}

		// not safe / strike 1
		// try removing the problem number
		for (int i = 0; i < lv.size(); ++i) {
			std::vector<int> lv2;
			for (int j = 0; j < lv.size(); ++j) {
				if (j == i)
					continue;
				lv2.push_back(lv[j]);
			}
			if (get_problem(lv2) == -1) {
				++res;
				break;
			}
		}
	}

	// print
	std::cout << res << std::endl;
}