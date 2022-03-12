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
        Node *parent;
        Node *left;
        Node *right;
        color_t color;
    public:
        //Constructors
        Node(T key, color_t color);
        Node(T key);

        // Getters
        T getData() {return data;}
        Node* getParent(){return parent;}
        Node* getLeft(){return left;}
        Node* getRight(){return right;}
        color_t getColor(){return color;}

        // Setters
        void setData(T a){data = a;}
        void setParent(Node* a){parent = a;}
        void setLeft(Node* a){left = a;}
        void setRight(Node* a){right = a;}
        void setColor(color_t a){color = a;}

        bool hasRight();
        bool hasLeft();
        bool hasParent();

        // Operator Overloading
        inline bool operator==(const Node& l){
            return data == l.data;
        }
};

template<typename T>
Node<T>::Node(T key, color_t init_color){
    data = key;
    color = init_color;
    parent = nullptr;
    left = nullptr;
    right = nullptr;
}

template<typename T>
Node<T>::Node(T key){
    data = key;
    color = black;
    parent = nullptr;
    left = nullptr;
    right = nullptr;
}

template<typename T>
bool Node<T>::hasRight(){
    return this->right != nullptr;
}


template<typename T>
bool Node<T>::hasLeft(){
    return this->left != nullptr;
}

template<typename T>
bool Node<T>::hasParent(){
    return this->parent != nullptr;
}

#endif //GRAPH_NODE_HPP
