// SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
//
// SPDX-License-Identifier: Apache-2.0

#include "PiccoloGatewayGearComparator.h"

PiccoloGatewayGearComparator::PiccoloGatewayGearComparator(class PiccoloGatewayManager* m, class PiccoloEvent* event)
{
	manager_ = m;
	pe_ = event;
}
PiccoloGatewayGearComparator::~PiccoloGatewayGearComparator()
{

}

void PiccoloGatewayGearComparator::compare(void* data)
{
	std::cout << "GearComparator compare called" << std::endl;
	gearState::DataType* receiveData = static_cast<gearState::DataType*>(data);

	checkCondition(std::string(receiveData->gear()));
}

void PiccoloGatewayGearComparator::checkCondition(std::string gearState)
{
	std::string targetValue = pe_->targetValue;
	std::string express = pe_->express;

	if(express.compare(std::string("Equal")) == 0 )
	{
		if(targetValue.compare(gearState) == 0 )
		{
			manager_->comparatorCallback(pe_->name, pe_->topic);
		}
	}else if(express.compare(std::string("NotEqual")) == 0 )
	{
		if(targetValue.compare(gearState) != 0 )
		{
			manager_->comparatorCallback(pe_->name, pe_->topic);
		}
	}else{
		std::cout <<"wrong express" <<std::endl;
	}
}
