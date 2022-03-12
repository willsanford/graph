#ifndef POINT_H
#define POINT_H

class Point{
    private:
        float x;
        float y;

    public:
        Point();
        Point(float x, float y);
        void setY(float y);
        void setX(float x);
        float getY();
        float getX();
};
#endif