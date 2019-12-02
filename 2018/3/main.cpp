#include <iostream>
#include <vector>
#include <functional>

struct rectangle
{
	int id;
	int left;
	int top;
	int width;
	int height;

	void print() const
	{
		std::cout << "#" << id << " @ " << left << "," << top << ": " << width << "x" << height << std::endl;
	}

	void loop_area(std::function<void(int, int)> fn) const
	{
		for(int i=left; i<left+width; ++i)
		{
			for(int j=top; j<top+height; ++j)
			{
				fn(i, j);
			}
		}
	}
};

int main()
{
	std::vector<rectangle> inputs;
	rectangle current;
	char hash;
	char at;
	char comma;
	char colon;
	char times;
	while(std::cin >> hash >> current.id >> at >> current.left >> comma >> current.top >> colon >> current.width >> times >> current.height)
	{
		inputs.push_back(current);
	}

	int overlapping = 0;
	std::vector<std::vector<int>> area;

	for(const auto& input : inputs)
	{
		//input.print();

		auto mark_overlaps = [&area, &overlapping](int i, int j){
				if(area.size() <= i)
					area.resize(i + 1);

				if(area[i].size() <= j)
					area[i].resize(j +1);

				if(area[i][j] == 1)
					++overlapping;

				++area[i][j];
		};

		input.loop_area(mark_overlaps);
	}

	std::cout << overlapping << " overlapping square inches" << std::endl;

	for(const auto& input : inputs)
	{
		bool overlaps = false;
		auto check_overlaps = [&area, &overlaps](int i, int j){
			if(area[i][j] > 1)
				overlaps = true;
		};

		input.loop_area(check_overlaps);

		if(!overlaps)
			std::cout << input.id << " does not overlap" << std::endl;
	}

	return 0;
}
