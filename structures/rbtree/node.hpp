//
// Created by wsanf on 3/12/22.
//

#ifndef GRAPH_NODE_HPP
#define GRAPH_NODE_HPP

enum color_t{red, black};

template<typename T>
class Node {
    private:
        T data;
        Node<T> *parent = nullptr;
        Node<T> *left = nullptr;
        Node<T> *right = nullptr;
        color_t color = red;
    public:
        //Constructors
        Node(T key);

        // Getters
        T getData() {return data;}
        Node* getParent(){return parent;}
        Node* getLeft(){return left;}
        Node* getRight(){return right;}
        color_t getColor(){return color;}

        // Setters
        void setData(T a){data = a;}
        void setParent(Node<T>* a){parent = a;}
        void setLeft(Node<T>* a){left = a;}
        void setRight(Node<T>* a){right = a;}
        void setColor(color_t a){color = a;}

        bool hasRight();
        bool hasLeft();
        bool hasParent();

        void flipColor();
        bool isLeftChild();
        bool isRightChild();

        // Operator Overloading
        inline bool operator==(const Node& l){
            return data == l.data;
        }
};

template<typename T>
Node<T>::Node(T key) {
    data = key;
}

template<typename T>
bool Node<T>::hasRight(){
    return !(this->right == nullptr);
}


template<typename T>
bool Node<T>::hasLeft(){
    return !(this->left == nullptr);
}

template<typename T>
bool Node<T>::hasParent(){
    return !(this->parent == nullptr);
}

template<typename T>
void Node<T>::flipColor() {
    if (this->color == red){
        this->color = black;
    }else{
        this->color = red;
    }
}

template<typename T>
bool Node<T>::isLeftChild() {
    if (!this->parent->hasLeft()){return false;}
    if (this == this->parent->getLeft()){ return true;}
    return false;
}

template<typename T>
bool Node<T>::isRightChild(){
    if (!this->parent->hasRight()){return false;}
    if (this == this->parent->getRight()){ return true;}
    return false;
}
#endif //GRAPH_NODE_HPP
