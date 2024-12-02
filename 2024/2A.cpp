#include <bits/stdc++.h>
#include "readfile.hpp"


int main(int argc, char const *argv[])
{
	int res = 0;
	std::vector<std::string> file = readfile("2A.in");
	if (file.empty())
		return 1;

	for (const auto &line : file) {
		std::istringstream iss(line);
		int n;
		bool safe = true;
		int prev = -1;
		bool increasing = false, decreasing = false;
		// put to vector and pair of two
		while (iss >> n) {
			if (prev == -1) {
				prev = n;
				continue;
			}
			if (prev < n)
				increasing = true;
			else if (prev > n)
				decreasing = true;
			if (std::abs(prev - n) > 3 || prev == n || (n > prev && decreasing || n < prev && increasing || increasing && decreasing || !increasing && !decreasing)) {
				safe = false;
				break;
			}
			prev = n;
		}
		if (safe) {
			res++;
		}
	}

	// print
	std::cout << res << std::endl;
}