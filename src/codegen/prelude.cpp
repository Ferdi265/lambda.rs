#include <memory>
#include <vector>

class lambda {
    std::shared_ptr<std::vector<lambda>> captures;
    lambda (*function)(lambda[], lambda);

public:
    lambda(lambda (*f)(lambda[], lambda), std::initializer_list<lambda> c)
        : captures(std::make_shared<std::vector<lambda>>(c)), function(f) {}

    lambda operator()(lambda arg) {
        return function(&captures->front(), arg);
    }
};

