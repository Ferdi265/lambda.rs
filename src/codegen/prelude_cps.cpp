#include <cstddef>
#include <cstdint>
#include <array>
#include <algorithm>

struct Lambda;
struct Cont;
typedef Lambda* LambdaFn(Lambda*, Lambda*, Cont*);

class Lambda {
    friend class Cont;

    size_t refcount;
    LambdaFn* function;

    template <size_t N>
    Lambda(LambdaFn* f, std::array<Lambda*, N> cs, size_t value) :
        refcount(1),
        function(f),
        length(N)
    {
        std::copy_n(cs.data(), N, captures);
        data() = value;
    }

    static Lambda* nop_lambda(Lambda* arg, Lambda* self, Cont* cont);
    static Lambda* ret_lambda(Lambda* arg, Lambda* self, Cont* cont);

public:
    size_t length;
    Lambda* captures[];

    Lambda* ref(size_t n = 1) {
        refcount += n;
        return this;
    }

    void unref() {
        if (!--refcount) {
            for (size_t i = 0; i < length; i++) {
                captures[i]->unref();
            }

            delete[] (uint8_t*) this;
        }
    }

    size_t& data() {
        return (size_t&) captures[length];
    }

    Lambda* call(Lambda* arg, Cont* cont) {
        return function(arg, this, cont);
    }

    Lambda* ret(Lambda* arg = nullptr);

    template <size_t N>
    static Lambda* mk(LambdaFn* f, std::array<Lambda*, N> captures, size_t value = 0) {
        uint8_t * buf = new uint8_t[sizeof (Lambda) + (N + 1) * sizeof (Lambda*)];
        return new (buf) Lambda(f, captures, value);
    }
};

class Cont {
    Cont* continuation;
    Lambda* lambda;

    template <size_t N>
    Cont(LambdaFn* f, std::array<Lambda*, N> cs, Cont* cc) :
        continuation(cc),
        lambda(Lambda::mk(f, cs))
    {}

public:
    Lambda* call(Lambda* arg) {
        Lambda* l = lambda;
        Cont* c = continuation;
        delete this;
        return l->call(arg, c);
    }

    template <size_t N>
    static Cont* mk(LambdaFn* f, std::array<Lambda*, N> cs, Cont* cc) {
        return new Cont(f, cs, cc);
    }
};

Lambda* Lambda::nop_lambda(Lambda* arg, Lambda* self, Cont* cont) {
    self->unref();
    return cont->call(arg);
}

Lambda* Lambda::ret_lambda(Lambda* arg, Lambda* self, Cont* cont) {
    self->unref();
    return arg;
}

Lambda* Lambda::ret(Lambda* arg) {
    if (arg == nullptr) {
        arg = Lambda::mk<0>(Lambda::nop_lambda, {});
    }

    return this->call(arg, Cont::mk<0>(Lambda::ret_lambda, {}, nullptr));
}

