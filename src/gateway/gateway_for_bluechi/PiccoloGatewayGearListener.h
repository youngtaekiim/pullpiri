// SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
//
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef _PiccoloGatewayGearListener_
#define _PiccoloGatewayGearListener_

#include "PiccoloGatewayManager.h"
#include "PiccoloGatewayDDSListener.h"
#include "gearState.hpp"

class PiccoloGatewayGearListener : public PiccoloGatewayDDSListener
{
	public:
		PiccoloGatewayGearListener(class PiccoloGatewayManager* m);
		virtual ~PiccoloGatewayGearListener();

		virtual void run() override;

	private:
		class PiccoloGatewayManager* manager_;
		bool keepRunning_;
};

#endif

