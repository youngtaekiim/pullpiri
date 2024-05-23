// SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
//
// SPDX-License-Identifier: Apache-2.0

#include "PiccoloGatewayManager.h"
#include <iostream>

PiccoloGatewayManager::PiccoloGatewayManager()
{
	std::cout << "manager creator" << std::endl;
}

PiccoloGatewayManager::~PiccoloGatewayManager()
{
	for(auto e : ddsListenerMap_)
	{
		e.second->keepRunning_ = false;
	}
	for(const auto e : eventMap_)
	{
		delete e.second;
	}
	for(auto& e : thVec_)
	{
		e.join();
	}
	for(const auto e : ddsListenerMap_)
	{
		delete e.second;
	}
}

void PiccoloGatewayManager::setGrpcCaller(class PiccoloGatewayStateManagerCaller* caller)
{
	smCaller_ = caller;
}

void PiccoloGatewayManager::grpcCalled(int command, std::string key, int target)
{
	if(command == 0){
		std::cout << "Manager grpcCalled" <<std::endl;
		PiccoloEvent* pe = new PiccoloEvent();
		pe->name = key;
		pe->targetDest = target;
		PiccoloGatewayParser::parse(pe);
		setEvent(pe);
	}else if(command == 1){
		removeEvent(key);
	}
}

void PiccoloGatewayManager::ddsReceived(void* data, std::string topic)
{
	for (auto iter = eventComparatorMap_.begin() ; iter !=  eventComparatorMap_.end(); iter++)
	{
		for(auto e : iter->second)
		{
			if(e.first.compare(topic) == 0 )
			{
				std::cout << "send data" << std::endl;
				PiccoloGatewayComparator* p = e.second;
				std::thread([p, data](){p->compare(data);}).detach();
			}
		}	
	}
}

void PiccoloGatewayManager::setEvent(PiccoloEvent* event)
{
	std::cout << "Manager set event called" << std::endl;
	std::map<std::string, PiccoloEvent*>::iterator eventIt;
	// mutex locked via grpc server
        eventIt = (eventMap_).find(event->name);
        if(eventIt != (eventMap_).end()){
		removeEvent(event->name);
	}
	(eventMap_).insert(std::make_pair(event->name, event));
	std::map<std::string, PiccoloGatewayDDSListener*>::iterator DDSIt;
        DDSIt = ddsListenerMap_.find(event->topic);
	if(DDSIt == ddsListenerMap_.end()){
		setDDSlistenerByTopic(event->topic);
        }
	setEventComparator(event);
}

void PiccoloGatewayManager::removeEvent(std::string eventName)
{
	std::cout << "Manager remove event called" << std::endl;
	std::map<std::string, PiccoloEvent*>::iterator it;
        it = (eventMap_).find(eventName);
        if(it != (eventMap_).end()){
		delete it->second;
		(eventMap_).erase(it);
        }
	std::map<std::string, std::map<std::string, PiccoloGatewayComparator*>>::iterator comaparatorIt;
	comaparatorIt = eventComparatorMap_.find(eventName);
	if(comaparatorIt != eventComparatorMap_.end()){
		for(auto e : comaparatorIt->second)
		{
			delete e.second;
		}
                eventComparatorMap_.erase(comaparatorIt);
        }
}


void PiccoloGatewayManager::setDDSlistenerByTopic(std::string topic)
{
	std::cout << "Manager setDDSlistenerByTopic called" << std::endl;
	if(topic.compare(std::string("rt/piccolo/gear_state")) == 0 )
	{
		PiccoloGatewayDDSListener* pdl = new PiccoloGatewayGearListener(this);
		ddsListenerMap_.insert(std::make_pair(topic, pdl));
		thVec_.emplace_back(&PiccoloGatewayGearListener::run, static_cast<PiccoloGatewayGearListener*>(pdl));
	}else{
		std::cout << "wrong dds topic" << std::endl;
	}
}

void PiccoloGatewayManager::setEventComparator(PiccoloEvent* event)
{
	//TO-DO make multi condition 
	if(event->topic.compare(std::string("rt/piccolo/gear_state")) == 0)
	{
		PiccoloGatewayComparator* comp = new PiccoloGatewayGearComparator(this, event);
		std::map<std::string, PiccoloGatewayComparator*> forwardMap;
		forwardMap.insert(std::make_pair(event->topic, comp));
		eventComparatorMap_.insert(std::make_pair(event->name, forwardMap));
	}else{
		std::cout << "wrong dds topic" << std::endl;
	}
}

void PiccoloGatewayManager::comparatorCallback(std::string name, std::string topic)
{
	std::cout << "comparatorCallback called" << std::endl;
	PiccoloEvent* event = eventMap_[name];
	smCaller_->Send(std::string(event->actionKey));
	
	if(event->lifeCycle.compare(std::string("oneTime")) == 0 ) removeEvent(name);
}
