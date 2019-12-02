#include <iostream>
#include <vector>

struct position
{
	int64_t x;
	int64_t y;
};

struct properties
{
	int64_t geologic;
	int64_t erosion;
	int type;

	std::string to_string() const
	{
		switch(type % 3)
		{
			case 0:
				return ".";
			case 1:
				return "=";
			case 2:
				return "|";
			default:
				throw std::runtime_error("unexpected type");
		}
	}
};

int main()
{
	int64_t depth;
	position target;

	std::string d;
	char comma;
	std::cin >> d >> depth;
	std::cin >> d >> target.x >> comma >> target.y;

	std::cout << depth << std::endl;
	std::cout << target.x << ", " << target.y << std::endl;

	std::vector<std::vector<properties>> grid(target.y+1, std::vector<properties>(target.x+1));

	int64_t risk_level = 0;

	for(int64_t y=0; y<=target.y; ++y)
	{
		for(int64_t x=0; x<=target.x; ++x)
		{
			properties props;
			if(x == target.x && y == target.y)
			{
				props.geologic = 0;
			}
			else if(y == 0)
			{
				props.geologic = x * 16807;
			}
			else if(x == 0)
			{
				props.geologic = y * 48271;
			}
			else
			{
				props.geologic = grid[y-1][x].erosion * grid[y][x-1].erosion;
			}

			props.erosion = (props.geologic + depth) % 20183;
			props.type = props.erosion % 3;
			grid[y][x] = props;

			risk_level += props.type;
		}
	}

	for(const auto& row : grid)
	{
		for(const auto& cell : row)
		{
			std::cout << cell.to_string();
		}
		std::cout << std::endl;
	}

	std::cout << "risk level: " << risk_level << std::endl;

	return 0;
}
