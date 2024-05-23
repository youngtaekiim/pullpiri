// SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
//
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef _PiccoloGatewayGearComparator_
#define _PiccoloGatewayGearComparator_

#include "PiccoloGatewayManager.h"
#include "PiccoloGatewayComparator.h"
#include "gearState.hpp"
#include <thread>

class PiccoloGatewayGearComparator : public PiccoloGatewayComparator
{
        public:
                PiccoloGatewayGearComparator(class PiccoloGatewayManager* m, class PiccoloEvent* event);
                virtual ~PiccoloGatewayGearComparator();

                virtual void compare(void* data) override;

		class PiccoloEvent* pe_;
		std::string name_;

        private:
		void checkCondition(std::string gearState);
                class PiccoloGatewayManager* manager_;
};

#endif

