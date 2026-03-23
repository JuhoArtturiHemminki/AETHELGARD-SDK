#ifndef AETHELGARD_H
#define AETHELGARD_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef struct AethelgardV2 AethelgardV2;

AethelgardV2* aethelgard_create(size_t order, float lr);

void aethelgard_process(AethelgardV2* ptr, 
                        float dirty, 
                        float clean, 
                        float* out_est, 
                        float* out_err);

void aethelgard_destroy(AethelgardV2* ptr);

#ifdef __cplusplus
}
#endif

#endif

