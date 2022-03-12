#ifndef SEGMENT_H
#define SEGMENT_H

#include "point.hpp"


class Segment{
    private:
        Point start;
        Point end;
    public:
        Segment();
        Segment(Point start, Point end);


};
#endif