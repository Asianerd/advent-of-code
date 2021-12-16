from enum import Enum

raw_data_string = """
D2FE28
38006F45291200
EE00D40C823060
"""


class Data:
    def __init__(self, raw):
        self.bin_string = raw


class Packet:
    packets = []

    def __init__(self, raw):
        self.hex_string = raw
        self.bin_string = Packet.hextobin(self.hex_string)

        self.version = int(self.bin_string[0:3], 2)
        self.id = int(self.bin_string[3:6], 2)

        self.type = Packet.Type.Literal if (self.id == 4) else Packet.Type.Operator

        if self.type == Packet.Type.Operator:
            self.length_type = Packet.DataLengthType.bit11 if (self.bin_string[6] == '1') else Packet.DataLengthType.bit15
            if self.length_type == Packet.DataLengthType.bit11:
                self.data_length = self.bin_string[7:18]
            else:
                self.data_length = self.bin_string[7:22]
            print(f"{self.length_type} : {self.data_length}")
        else:
            self.data = []
            _data_location = self.bin_string[6:-1]
            for index in range(int(len(_data_location) / 5)):
                self.data.append(Data(_data_location[index * 5:(index * 5) + 5]))

        print(f"{self.bin_string} {self.version} {self.id} {self.type}")

    @staticmethod
    def extract_packets(packet):
        packets_found = []
        

    @staticmethod
    def hextobin(h):
        return bin(int(h, 16))[2:].zfill(len(h) * 4)

    class Type(Enum):
        Literal = 0
        Operator = 1

    class DataLengthType(Enum):
        bit15 = 0  # Next 15 bits - total length of data (in bits)
        bit11 = 1  # Next 11 bits - number of data that is available


for x in raw_data_string.strip().split("\n"):
    Packet.packets.append(Packet(x))
