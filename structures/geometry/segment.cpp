#include "segment.hpp"

// Default constructor
Segment::Segment(){
    this->start = Point();
    this->end = Point();
}

Segment::Segment(Point start, Point end){
    this->start = start;
    this->end = end;
}