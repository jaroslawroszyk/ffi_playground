#include <complex>
#include <cmath>
#include <cstdlib>

extern "C" {

void fft(const double* input_real, const double* input_imag,
         double* out_real, double* out_imag, int n) 
{
    for(int k=0; k<n; k++) {
        std::complex<double> sum(0,0);
        for(int t=0; t<n; t++) {
            double angle = -2 * M_PI * t * k / n;
            std::complex<double> exp_term(cos(angle), sin(angle));
            sum += std::complex<double>(input_real[t], input_imag[t]) * exp_term;
        }
        out_real[k] = sum.real();
        out_imag[k] = sum.imag();
    }
}

}
