# SPDX-FileCopyrightText: Copyright 2024 LG Electronics Inc.
# SPDX-License-Identifier: Apache-2.0

import sys
from cyclonedds.domain import DomainParticipant
from cyclonedds.pub import DataWriter
from cyclonedds.topic import Topic
from cyclonedds.util import duration

import gearState
from gearState import DataType

if __name__ == '__main__':
        participant = DomainParticipant()
        topic = Topic(participant, "rt/piccolo/gear_state", DataType)
        writer = DataWriter(participant, topic)

        # 'drive', 'parking', 'neutral', 'reverse'
        data = gearState.DataType('reverse')
        writer.write(data)