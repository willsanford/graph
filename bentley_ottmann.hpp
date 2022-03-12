#ifndef BENTLY_OTTMANN_H
#define BENTLY_OTTMANN_H


#include <cstdlib>
#include <vector>
#include "segment.hpp"
#include "tree.hpp"


/*
* Run the bentley ottmann algorithm on a vector of input segments
*/
std::vector<Point> bentley_ottmann(std::vector<Segment>& segments);





#endif