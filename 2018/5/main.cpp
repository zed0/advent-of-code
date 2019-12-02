#include <iostream>
#include <algorithm>
#include <cmath>

int collapsed_size(std::string in)
{
	for(int i=0; i < in.size() - 1; ++i)
	{
		if(abs(in[i]-in[i+1]) == 'a'-'A')
		{
			in.erase(i,2);
			if(i-->0) i-=1;
		}
	}
	return in.size();
}

int main()
{
	std::string input;
	std::cin >> input;

	std::cout << "Part 1: " << collapsed_size(input) << " remaining" << std::endl;

	int best = 999999999;
	for(char c='A'; c<='Z'; ++c)
	{
		std::string current = input;
		current.erase(std::remove(current.begin(), current.end(), c), current.end());
		current.erase(std::remove(current.begin(), current.end(), c+'a'-'A'), current.end());
		int length = collapsed_size(current);
		if(length < best)
			best = length;
	}

	std::cout << "Part 2: " << best << " remaining" << std::endl;

	return 0;
}
