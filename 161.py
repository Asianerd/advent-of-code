from enum import Enum

raw_data_string = """
D2FE28
38006F45291200
EE00D40C823060
"""


class Packet:
    packets = []

    def __init__(self, raw):
        self.bin_string = raw
        self.version = self.bin_string[0:3]
        self.id = self.bin_string[3:6]
        self.type = Packet.Type.Literal if self.id == '100' else Packet.Type.Operator
        self.data = []

        self.data_length_type = Packet.LengthType.bit15
        self.data_length = 0

        if self.type == Packet.Type.Literal:
            self.initialize_literal()
        else:
            self.initialize_operator()

        print(f"Type : {self.type.name}\nVersion : {self.version}\nId : {self.id}\nData {' ; '.join(self.data)}\n\n")

    def initialize_literal(self):
        data_string = self.bin_string[6:-1]
        is_end = False
        for index in range(int(len(data_string) / 5)):
            if data_string[index * 5] == '0':
                is_end = True
            self.data.append(data_string[index * 5:(index * 5) + 5])
            if is_end:
                break

    def initialize_operator(self):
        self.data_length_type = Packet.LengthType.bit15 if self.bin_string[6] == '0' else Packet.LengthType.bit11
        if self.data_length_type == Packet.LengthType.bit15:
            self.data_length = int(self.bin_string[7:22], 2)
            self.initialize_bit15()
        else:
            self.data_length = int(self.bin_string[7:18], 2)
            self.initialize_bit11()

    def initialize_bit15(self):
        data_range = self.bin_string[22:22 + self.data_length]

    def initialize_bit11(self):
        data_range = self.bin_string[18:-1]

    @staticmethod
    def hex_to_bin(h):
        return bin(int(h, 16))[2:].zfill(len(h) * 4)

    class Type(Enum):
        Literal = 0
        Operator = 1

    class LengthType(Enum):
        bit11 = 0
        bit15 = 1


for x in raw_data_string.strip().split("\n"):
    binary = Packet.hex_to_bin(x)
    Packet.packets.append(Packet(str(binary)))

    # id = int(binary[3:6], 2)
    #
    # if id == 4:
    #     # Literal
    #     Packet.packets.append(Packet(str(binary)))
    # else:
    #     # Operator
    #     pass

