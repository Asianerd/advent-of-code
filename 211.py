raw_data_string = """
Player 1 starting position: 2
Player 2 starting position: 10
"""


class Player:
    dice_rolls = 0
    current_dice = 0
    players = []
    run = True

    def __init__(self, starting):
        self.space = starting
        self.score = 0

    def progress_spaces(self, amount):
        self.space += amount
        while self.space > 10:
            self.space -= 10
        self.score += self.space

    @staticmethod
    def static_update():
        for x in Player.players:
            final_dice = 0
            for iteration in range(3):
                Player.current_dice += 1
                Player.dice_rolls += 1
                final_dice += Player.current_dice
            x.progress_spaces(final_dice)

            if len([x for x in Player.players if x.score >= 1000]) >= 1:
                Player.run = False

            if not Player.run:
                return
            print(f"End:\t{x.space}\t{x.score}")


for x in raw_data_string.strip().split("\n"):
    starting_space = int(x.split(":")[-1].strip())
    Player.players.append(Player(starting_space))

while Player.run:
    Player.static_update()
    # if len([x for x in Player.players if x.score >= 1000]) >= 1:
    #     break

loser = [x for x in Player.players if x.score < 1000][0]
print(f"Rolls : {Player.dice_rolls}\t{loser.score}\t{loser.score * Player.dice_rolls}")
