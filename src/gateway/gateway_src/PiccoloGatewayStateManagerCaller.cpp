/*
 *
 * Copyright 2015 gRPC authors.
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 *
 */

/*
 * Copyright (c) 2024 LG Electronics Inc.
 * SPDX-License-Identifier: Apache-2.0
 */


#include "PiccoloGatewayStateManagerCaller.h"
/*
using grpc::Channel;
using grpc::ClientAsyncResponseReader;
using grpc::ClientContext;
using grpc::CompletionQueue;
using grpc::Status;
using statemanager::Connection;
using statemanager::SendResponse;
using statemanager::SendRequest;
*/
bool PiccoloGatewayStateManagerCaller::Send(const std::string& key)
{
	// Data we are sending to the server.
	statemanager::SendRequest sr;
	sr.set_from(constants::PiccoloModuleName::gateway);
	sr.set_request(key);

	// Container for the data we expect from the server.
	statemanager::SendResponse respons;

	// Context for the client. It could be used to convey extra information to
	// the server and/or tweak certain RPC behaviors.
	grpc::ClientContext context;

	// The producer-consumer queue we use to communicate asynchronously with the
	// gRPC runtime.
	grpc::CompletionQueue cq;

	// Storage for the status of the RPC upon completion.
	grpc::Status status;

	std::unique_ptr<grpc::ClientAsyncResponseReader<statemanager::SendResponse> > rpc(
			stub_->AsyncSend(&context, sr, &cq));

	// Request that, upon completion of the RPC, "reply" be updated with the
	// server's response; "status" with the indication of whether the operation
	// was successful. Tag the request with the integer 1.
	rpc->Finish(&respons, &status, (void*)1);
	void* got_tag;
	bool ok = false;
	// Block until the next result is available in the completion queue "cq".
	// The return value of Next should always be checked. This return value
	// tells us whether there is any kind of event or the cq_ is shutting down.
	GPR_ASSERT(cq.Next(&got_tag, &ok));

	// Verify that the result from "cq" corresponds, by its tag, our previous
	// request.
	GPR_ASSERT(got_tag == (void*)1);
	// ... and that the request was completed successfully. Note that "ok"
	// corresponds solely to the request for updates introduced by Finish().
	GPR_ASSERT(ok);

	// Act upon the status of the actual RPC.
	if (status.ok()) {
	  return true;
	} else {
	  return false;
	}
}


