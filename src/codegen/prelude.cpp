#include <functional>

class lambda {
    std::function<lambda(lambda)> function;

public:
    template <typename F>
    lambda(F f) : function(f) {}

    lambda operator()(lambda arg) const {
        return function(arg);
    }
};

