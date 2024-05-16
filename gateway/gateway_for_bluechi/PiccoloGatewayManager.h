// SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
//
// SPDX-License-Identifier: Apache-2.0

#pragma once

#ifndef _PiccoloGatewayManager_
#define _PiccoloGatewayManager_

#include <map>
#include <thread>
#include <string>
#include <mutex>
#include <condition_variable>
#include <memory>
#include <vector>
#include <functional>
#include "PiccoloEvent.h"
#include "PiccoloGatewayParser.h"
#include "PiccoloGatewayDDSListener.h"
#include "PiccoloGatewayGearListener.h"
#include "PiccoloGatewayServer.h"
#include "PiccoloGatewayComparator.h"
#include "PiccoloGatewayGearComparator.h"
#include "PiccoloGatewayStateManagerCaller.h"


class PiccoloGatewayManager
{
	public:
		PiccoloGatewayManager();
		virtual ~PiccoloGatewayManager();
		void setGrpcCaller(class PiccoloGatewayStateManagerCaller* caller);
		void grpcCalled(int command, std::string key, int target);
		void ddsReceived(void* data, std::string topic);
		void comparatorCallback(std::string name, std::string topic);

	private:
		void setEvent(PiccoloEvent* event);
		void removeEvent(std::string eventName);

		void setDDSlistenerByTopic(std::string topic);
		void setEventComparator(PiccoloEvent* event);

		std::map<std::string, PiccoloEvent*> eventMap_;
		std::map<std::string, PiccoloGatewayDDSListener*> ddsListenerMap_;
		std::map<std::string, std::map<std::string, PiccoloGatewayComparator*>> eventComparatorMap_;
		std::vector<std::thread> thVec_;
		std::condition_variable managerCv_;
		class PiccoloGatewayStateManagerCaller* smCaller_;
};
#endif
