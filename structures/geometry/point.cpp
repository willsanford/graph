#include "point.hpp"
// Defualt constructor
Point::Point(){
    this->x = x;
    this->y = y;
}

Point::Point(float x, float y){
    this->x = x;
    this->y = y;
};

void Point::setY(float y){
    this->y = y;
};

void Point::setX(float x){
    this->x = x;
};

float Point::getX(){
    return this->x;
};
float Point::getY(){
    return this->y;
};
