
#include<math.h>

typedef double f64;
typedef long long i64;
typedef unsigned long long u64;
typedef long long bool;

#define REP(X) {X, X, X, X, X, X, X, X}
#define REINTERP(from, F, T) union { F f; T t; } u; u.f = from; return u.t;

inline f64 f64_mul_add(f64 a, f64 b, f64 c) {
    return a * b + c;
}

inline f64 f64_select(bool a, f64 b, f64 c) {
    return a ? b : c;
}

inline f64 f64_round(f64 a) {
    return round(a);
}

inline f64 f64_f(double f) {
    return (f64)f;
}

inline u64 f64_mkuty(long long v) {
    return (u64)v;
}

inline f64 f64_mkfty(long long v) {
    REINTERP(v, long long, double)
}

inline u64 f64_reinterpret_fty_uty(f64 f) {
    REINTERP(f, f64, u64)
}

inline f64 f64_reinterpret_uty_fty(u64 f) {
    REINTERP(f, u64, f64)
}

const f64 PI = M_PI;
const f64 LOG2_E = M_LOG2E;
const f64 LOG2_10 = M_LN10 / M_LN2;
const f64 MIN_POSITIVE = 2.2250738585072014E-308;
f64 f64_log2(f64 arg) {
  u64 arg_bits = f64_reinterpret_fty_uty(arg);
  u64 exponent = (arg_bits >> f64_mkuty(52ull)) - f64_mkuty(1023ull);
  f64 x = f64_reinterpret_uty_fty((arg_bits & f64_mkuty(4503599627370496ull - 1ull)) | f64_mkuty(4607182418800017408ull)) - f64_f(1.5);
  f64 y = f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add(f64_mul_add((-f64_mkfty(4546374278107678680ull)), x, f64_mkfty(4549472900654298694ull)), x, -f64_mkfty(4548163352978971471ull)), x, f64_mkfty(4550979242292870686ull)), x, -f64_mkfty(4555268298898397793ull)), x, f64_mkfty(4558493969767053974ull)), x, -f64_mkfty(4561674950935626047ull)), x, f64_mkfty(4564626928674686496ull)), x, -f64_mkfty(4567915592780460350ull)), x, f64_mkfty(4571457650865613616ull)), x, -f64_mkfty(4574762503947417879ull)), x, f64_mkfty(4578107178233634012ull)), x, -f64_mkfty(4581741589614488110ull)), x, f64_mkfty(4585636752412570626ull)), x, -f64_mkfty(4589798106271600139ull)), x, f64_mkfty(4594301705899034598ull)), x, -f64_mkfty(4599447016227734533ull)), x, f64_mkfty(4606838314010019088ull)), x, f64_mkfty(4603444093345823441ull));
  return y + ((f64)exponent);
}

f64 f64_ln(f64 arg) {
  return f64_log2(arg) * f64_f(1.0 / LOG2_E);
}

f64 f64_negate_on_odd(f64 x, f64 y) {
  u64 sign_bit = (u64)((((i64)x) & 1ull) << 63ull);
  return f64_from_bits(sign_bit ^ f64_to_bits(y));
}

f64 f64_recip_approx(f64 x) {
  f64 y = f64_from_bits((u64)((f64_mul_add(((f64)f64_to_bits(f64_abs(x))), -1.0, (f64)4607182418800017408ull * 2.0))));
  return f64_copysign((y - 0.08), x);
}

f64 f64_sqrt_approx(f64 x) {
  f64 y = f64_from_bits((u64)((f64_mul_add(((f64)f64_to_bits(f64_abs(x))), 0.5, (f64)4607182418800017408ull * 0.5))));
  return y - 0.08;
}

f64 f64_cbrt_approx(f64 x) {
  f64 y = f64_from_bits((u64)((f64_mul_add(((f64)f64_to_bits(f64_abs(x))), 1.0 / 3.0, (f64)4607182418800017408ull * 2.0 / 3.0))));
  return f64_copysign((y - 0.08), x);
}

