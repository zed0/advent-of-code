#include <iostream>
#include <algorithm>
#include <vector>

int main()
{
	int64_t val0 = 0;
	int64_t val1 = 0;
	int64_t val2 = 0;
	int64_t val3 = 65536;
	// val4 is ip
	// Rhiba's number
	//int64_t original_num = 733884;
	// Ben's number
	int64_t original_num = 2176960;
	int64_t part_1 = 11474091;
	int64_t val5 = original_num;
	//int64_t last = -1;

	std::vector<int64_t> seen;
	int64_t loop = 0;
	while(true)
	{
		if(++loop %1000000 == 0)
		{
			std::cout << loop << std::endl;
		}
		val1 = val3 & 255;
		val5 = val1 + val5;
		val5 = val5 & 16777215;
		val5 *= 65899;
		val5 = val5 & 16777215;

		if(256 > val3)
		{
			if(std::find(seen.begin(), seen.end(), val5) != seen.end())
			{
				/*
				for(const auto& val : seen)
				{
					std::cout << val << ", ";
				}
				std::cout << std::endl;
				*/
				//break;
			}
			else
			{
				seen.push_back(val5);
				std::cout << "val5: " << val5 << std::endl;
			}
			//std::cout << seen.size() << " values (last = " << seen.back() << ")" << std::endl;
			val1 = 0;
			val3 = val5 | 65536;
			val5 = original_num;
		}
		else
		{
			val1 = (val3/256) + 1;
			if(val1*256 > val3)
			{
				val1--;
			}
			if((val1+1)*256 <= val3)
			{
				val1++;
			}
			val3 = val1;
		}
	}

	return 0;
}
