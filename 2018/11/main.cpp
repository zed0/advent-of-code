#include <iostream>
#include <vector>
#include <iomanip>

int size = 300;

int get_power(int x, int y, int serial)
{
	int64_t rack_id = x + 10;
	int64_t m       = (y * rack_id * rack_id) + (serial * rack_id);
	int64_t digit   = (m/100)%10;
	return digit - 5;
}

int main()
{
	int64_t serial = 7400;

	std::vector<std::vector<int64_t>> grid(size, std::vector<int64_t>(size, 0));
	for(int x=0; x<size; ++x)
	{
		for(int y=0; y<size; ++y)
		{
			grid[y][x] = get_power(x, y, serial);
		}
	}

	{
		std::cout << "Part 1:" << std::endl;

		std::pair<int, int> best_coord(0, 0);
		int best_value = -10000;
		for(int x=0; x<size-3; ++x)
		{
			for(int y=0; y<size-3; ++y)
			{
				int value =
					grid[y][x] + grid[y+1][x] + grid[y+2][x] +
					grid[y][x+1] + grid[y+1][x+1] + grid[y+2][x+1] +
					grid[y][x+2] + grid[y+1][x+2] + grid[y+2][x+2];

				if(value > best_value)
				{
					best_coord = {x, y};
					best_value = value;
				}
			}
		}

		std::cout << "Best coord: " << best_coord.first << ", " << best_coord.second << std::endl;
		std::cout << "Best value: " << best_value << std::endl;
	}

	{
		std::cout << "Part 2:" << std::endl;

		std::pair<int, int> best_coord(0, 0);
		int best_size = 0;
		int best_value = -10000;
		for(int x=0; x<size; ++x)
		{
			for(int y=0; y<size; ++y)
			{
				int value = 0;
				for(int s=1; x+s < size && y+s < size; ++s)
				{
					for(int x_=x; x_<x+s; ++x_)
					{
						value += grid[y+s-1][x_];
					}
					for(int y_=y; y_<y+s-1; ++y_)
					{
						value += grid[y_][x+s-1];
					}

					if(value > best_value)
					{
						best_coord = {x, y};
						best_value = value;
						best_size = s;
					}
				}
			}
		}

		std::cout << "Best coord: " << best_coord.first << ", " << best_coord.second << std::endl;
		std::cout << "Best size: " << best_size << std::endl;
		std::cout << "Best value: " << best_value << std::endl;
	}

	return 0;
}
