// SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
//
// SPDX-License-Identifier: Apache-2.0

#include "PiccoloGatewayGearListener.h"

PiccoloGatewayGearListener::PiccoloGatewayGearListener(PiccoloGatewayManager* m)
{
	manager_ = m;
	keepRunning_ = true;
}

PiccoloGatewayGearListener::~PiccoloGatewayGearListener()
{
	keepRunning_ = false;
}

void PiccoloGatewayGearListener::run()
{
	std::cout << "Gear State listener thread run" << std::endl;
        std::string vehicleTopic("rt/piccolo/gear_state");

        dds::domain::DomainParticipant participant(org::eclipse::cyclonedds::domain::default_id());
        dds::topic::Topic<gearState::DataType> topic(participant, vehicleTopic);

        dds::sub::Subscriber subscriber(participant);
        dds::sub::DataReader<gearState::DataType> reader(subscriber, topic);

        while(keepRunning_)
        {
                auto received = reader.take();
                if ( received.length() > 0 )
                {
                        const gearState::DataType& msg = received.begin()->data();
                        const std::string receiveData = std::string(msg.gear());
                        if (receiveData.size() == 0) continue;

			gearState::DataType* pData = new gearState::DataType(msg);
			manager_->ddsReceived(static_cast<void*>(pData), vehicleTopic);

		}
		sleep(1);

	}

}
