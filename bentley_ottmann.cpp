#include <cstdlib>
#include <iostream>
#include <vector>
#include "tree.hpp"
#include "bentley_ottmann.hpp"

// Structure that we will use with our event point tree and pos queue
struct eventPoint{
    float x;
    float y;
    Segment* segment;

    // default + parameterized constructor
    eventPoint(float  x=0, float y=0, Segment* seg = nullptr) : x(x), y(y), segment(seg) {}

    eventPoint operator>(const eventPoint& a){
        return x > a.x;
    };

    eventPoint operator<(const eventPoint& a){
        return x < a.x;
    };

    eventPoint operator==(const eventPoint& a){
        return (x == a.x && y == a.y && segment == a.segment);
    }

    eventPoint operator=(const eventPoint& a){
        return eventPoint(a.x, a.y, a.segment);
    }
};

std::vector<Point> bentley_ottmann(std::vector<Segment>& segments){
    std::vector<Point> intersections = std::vector<Point>();

    // For each intersection, add an event point for both endpoints
    intersections.push_back(Point(0,0));
    return intersections;
};
