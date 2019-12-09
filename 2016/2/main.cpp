#include <iostream>
#include <vector>
#include <string>
#include <sstream>

int clamp(int in)
{
	if(in > 4)
		return 4;
	if(in < 0)
		return 0;
	return in;
}

int main()
{
	int x=0;
	int y=2;
	std::vector<std::vector<char>> board = {
		{' ', ' ', '1', ' ', ' '},
		{' ', '2', '3', '4', ' '},
		{'5', '6', '7', '8', '9'},
		{' ', 'A', 'B', 'C', ' '},
		{' ', ' ', 'D', ' ', ' '}
	};

	std::string line;
	while(std::getline(std::cin, line))
	{
		std::istringstream iss(line);
		char dir;
		while(iss >> dir)
		{
			int orig_x = x;
			int orig_y = y;
			switch(dir)
			{
				case 'U':
					y = clamp(y-1); break;
				case 'D':
					y = clamp(y+1); break;
				case 'L':
					x = clamp(x-1); break;
				case 'R':
					x = clamp(x+1); break;
			}
			if(board[y][x] == ' ')
			{
				x = orig_x;
				y = orig_y;
			}
		}
		std::cout << x << ',' << y << std::endl;
		std::cout << board[y][x] << std::endl;
	}
}
