import java.util.*;

class Blackjack {
    static String[] cardDeck, playerCards, dealerCards;
    static int money = 10, bet = 0, gamesWon = 0, gamesLost = 0;
    static int playerCardCount, dealerCardCount, deckIndex;

    static String[] cardArt = {
            "  _____\n |A .  |\n | /.\\ |\n |(_._)|\n |  |  |\n |____A|",
            "  _____\n |2    |\n |  ^  |\n |     |\n |  ^  |\n |____2|",
            "  _____\n |3    |\n | ^ ^ |\n |     |\n |  ^  |\n |____3|",
            "  _____\n |4    |\n | ^ ^ |\n |     |\n | ^ ^ |\n |____4|",
            "  _____\n |5    |\n | ^ ^ |\n |  ^  |\n | ^ ^ |\n |____5|",
            "  _____\n |6    |\n | ^ ^ |\n | ^ ^ |\n | ^ ^ |\n |____6|",
            "  _____\n |7    |\n | ^ ^ |\n |^ ^ ^|\n | ^ ^ |\n |____7|",
            "  _____\n |8    |\n |^ ^ ^|\n |^ ^ ^|\n |^ ^ ^|\n |____8|",
            "  _____\n |9    |\n |^ ^ ^|\n |^ ^ ^|\n |^ ^ ^|\n |____9|",
            "  _____\n |10 ^ |\n |^ ^ ^|\n |^ ^ ^|\n |^ ^ ^|\n |___10|",
            "  _____\n |J  ww|\n | ^ {)|\n |(.)%%|\n | |%%%|\n |_%%%>|\n",
            "  _____\n |Q  ww|\n | ^ {(|\n |(.)%%|\n | |%%%|\n |_%%%>|\n",
            "  _____\n |K  WW|\n | ^ {)|\n |(.)%%|\n | |%%%|\n |_%%%>|\n",
            "  _____\n |A ^  |\n | / \\ |\n | \\ / |\n |  .  |\n |____A|"
    };

    public static void main(String[] args) {
        Scanner input = new Scanner(System.in);
        printSplashScreen();
        delay();
        while (true) {
            System.out.print("Choose an option: (a)bout, (n)ew game, (h)elp: ");
            char choice = input.next().charAt(0);
            if (choice == 'a') {
                printAbout();
            } else if (choice == 'n') {
                startNewGame(input);
            } else if (choice == 'h') {
                printHelp();
            } else {
                System.out.println("Invalid option. Please choose again.");
            }
        }
    }

    static void startNewGame(Scanner input) {
        printWelcomeMessage();

        while (money > 0) {
            setupNewRound();
            printGameStatus();
            bet = getBet(input);
            money -= bet;
            System.out.println("Dealer shows: [" + dealerCards[0] + ", Hidden]");
            printPlayerCards();

            if (playerTurn(input)) {
                dealerTurn();
                determineWinner();
            }
        }
        printGameOver();
        System.out.print("Do you want to (t)ry again or (q)uit? ");
        char choice = input.next().charAt(0);
        if (choice == 't') {
            resetGame();
            startNewGame(input);
        } else {
            System.exit(0);
        }
    }

    static void printAbout() {
        System.out.println("Blackjack Game\nVersion 1.0\nDeveloped by 4t0m15");
    }

    static void printHelp() {
        System.out.println("Instructions:\n- (h)it: Draw another card.\n- (s)tand: Keep your current hand.\n- (d)ouble down: Double your bet and draw one final card.");
    }

    static void printSplashScreen() {
        System.out.println("""
                \033[1;35m\
                .------..------..------..------..------..------.        .------..------..------..------..------.
                |4.--. ||t.--. ||0.--. ||m.--. ||1.--. ||5.--. | .-.    |G.--. ||A.--. ||M.--. ||E.--. ||S.--. |
                | (\\/) || :/\\: || :/\\: || (\\/) || (\\/) || :/\\: |(())   | :/\\: || (\\/) || (\\/) || (\\/) || :/\\: |
                | :\\/: || (__) || :\\/: || :\\/: || :\\/: || :\\/: | '-.-.  | :\\/: || :\\/: || :\\/: || :\\/: || :\\/: |
                | '--'4|| '--'t|| '--'0|| '--'m|| '--'1|| '--'5|  (()) | '--'G|| '--'A|| '--'M|| '--'E|| '--'S|
                `------'`------'`------'`------'`------'`------'   '-'  `------'`------'`------'`------'`------'
                \033[0m""");
        delay();
        System.out.print("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n loading... \n");
    }

    static void printWelcomeMessage() {
        System.out.println("""
                \033[1;34m\
                ▀███████████▄   ▄█          ▄████████  ▄████████  ▄█   ▄█▄      ▄█    ▄████████  ▄████████    ▄█   ▄█▄\s
                  ███    ███ ███         ███    ███ ███    ███   ███ ▄███▀     ███     ███    ███ ███    ███   ███ ▄███▀\s
                  ███    ███ ███         ███    ███ ███    █▀    ███▐██▀       ███     ███    ███ ███    █▀    ███▐██▀  \s
                 ▄███▄▄▄██▀  ███         ███    ███ ███         ▄█████▀        ███     ███    ███ ███         ▄██████▀   \s
                  ▀▀███▀▀▀██▄  ███       ▀███████████ ███        ▀▀█████▄        ███   ▀███████████ ███        ▀▀█████▄   \s
                  ███    ██▄ ███         ███    ███ ███    █▄    ███▐██▄       ███     ███    ███ ███    █▄    ███▐██▄  \s
                  ███    ███ ███▌    ▄   ███    ███ ███    ████   ███ ▀███▄    ███    ███    ███ ███    ███   ███ ▀███▄\s
                ▄█████████▀  █████▄▄██   ███    █▀  ████████▀    ███   ▀█▀ █▄ ▄███     ███    █▀  ████████▀    ███   ▀█▀\s
                             ▀                                   ▀         ▀▀▀▀▀▀                              ▀        \s
                \033[0m""");
    }

    static void printGameStatus() {
        System.out.println("You have " + money + " coins");
        System.out.println("Games won: " + gamesWon + " | Games lost: " + gamesLost);
    }

    static void printPlayerCards() {
        System.out.print("Your cards: ");
        for (int i = 0; i < playerCardCount; i++) {
            System.out.print(playerCards[i] + " ");
        }
        System.out.println("\nYour total: " + getCardsValue(playerCards, playerCardCount));
        for (int i = 0; i < playerCardCount; i++) {
            System.out.println(cardArt[getCardIndex(playerCards[i])]);
        }
    }

    static int getBet(Scanner input) {
        while (true) {
            System.out.print("How many coins do you want to bet? ");
            if (input.hasNextInt()) {
                int bet = input.nextInt();
                if (bet <= money && bet > 0) return bet;
                System.out.println("Please bet between 1 and " + money + " coins.");
            } else {
                System.out.println("Invalid input. Please enter a number.");
                input.next();
            }
        }
    }

    static boolean playerTurn(Scanner input) {
        while (true) {
            System.out.print("Do you want to (h)it, (s)tand, or (d)ouble down? ");
            char choice = input.next().charAt(0);
            if (choice == 'h') {
                String newCard = cardDeck[deckIndex++];
                playerCards[playerCardCount++] = newCard;
                System.out.println("You got: " + newCard);
                printPlayerCards();
                if (getCardsValue(playerCards, playerCardCount) > 21) {
                    System.out.println("Bust! You went over 21!");
                    gamesLost++;
                    return false;
                }
            } else if (choice == 's') {
                return true;
            } else if (choice == 'd' && money >= bet) {
                money -= bet;
                bet *= 2;
                String newCard = cardDeck[deckIndex++];
                playerCards[playerCardCount++] = newCard;
                System.out.println("You got: " + newCard);
                printPlayerCards();
                if (getCardsValue(playerCards, playerCardCount) > 21) {
                    System.out.println("Bust! You went over 21!");
                    gamesLost++;
                    return false;
                }
                return true;
            } else {
                System.out.println("Please type 'h' for hit, 's' for stand, or 'd' for double down.");
            }
        }
    }

    static void dealerTurn() {
        System.out.println("Dealer's turn:");
        System.out.println("Dealer's cards: [" + dealerCards[0] + ", Hidden]");
        while (getCardsValue(dealerCards, dealerCardCount) < 17) {
            String newCard = cardDeck[deckIndex++];
            dealerCards[dealerCardCount++] = newCard;
            System.out.println("Dealer draws: " + newCard);
        }
        System.out.println("Dealer's cards: " + Arrays.toString(Arrays.copyOf(dealerCards, dealerCardCount)));
        for (int i = 0; i < dealerCardCount; i++) {
            System.out.println(cardArt[getCardIndex(dealerCards[i])]);
        }
    }
    static void determineWinner() {
        int playerTotal = getCardsValue(playerCards, playerCardCount);
        int dealerTotal = getCardsValue(dealerCards, dealerCardCount);
        System.out.println("Your total: " + playerTotal);
        System.out.println("Dealer's total: " + dealerTotal);
        if (dealerTotal > 21) {
            System.out.println("Dealer busts!");
            printPlayerWin();
            money += bet * 2;
            gamesWon++;
        } else if (playerTotal > dealerTotal) {
            printPlayerWin();
            money += bet * 2;
            gamesWon++;
        } else if (playerTotal < dealerTotal) {
            printDealerWin();
            gamesLost++;
        } else {
            System.out.println("It's a tie!");
            money += bet;
        }
    }

    static void setupNewRound() {
        cardDeck = createNewDeck();
        shuffleDeck(cardDeck);
        playerCards = new String[11];
        dealerCards = new String[11];
        playerCards[0] = cardDeck[0];
        playerCards[1] = cardDeck[1];
        dealerCards[0] = cardDeck[2];
        dealerCards[1] = cardDeck[3];
        playerCardCount = 2;
        dealerCardCount = 2;
        deckIndex = 4;
    }

    static String[] createNewDeck() {
        String[] newDeck = new String[52];
        String[] suits = {"Hearts", "Diamonds", "Clubs", "Spades"};
        String[] ranks = {"2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K", "A"};
        int index = 0;
        for (String suit : suits) {
            for (String rank : ranks) {
                newDeck[index++] = rank + " of " + suit;
            }
        }
        return newDeck;
    }

    static void shuffleDeck(String[] deck) {
        Random rand = new Random();
        for (int i = 0; i < deck.length; i++) {
            int randomIndex = rand.nextInt(deck.length);
            String temp = deck[i];
            deck[i] = deck[randomIndex];
            deck[randomIndex] = temp;
        }
    }

    static int getCardsValue(String[] cards, int count) {
        int total = 0, numberOfAces = 0;
        for (int i = 0; i < count; i++) {
            String rank = cards[i].split(" ")[0];
            if (rank.equals("A")) {
                numberOfAces++;
                total += 11;
            } else if (rank.equals("K") || rank.equals("Q") || rank.equals("J")) {
                total += 10;
            } else {
                total += Integer.parseInt(rank);
            }
        }
        while (total > 21 && numberOfAces > 0) {
            total -= 10;
            numberOfAces--;
        }
        return total;
    }

    static int getCardIndex(String card) {
        String rank = card.split(" ")[0];
        return switch (rank) {
            case "A" -> 0;
            case "2" -> 1;
            case "3" -> 2;
            case "4" -> 3;
            case "5" -> 4;
            case "6" -> 5;
            case "7" -> 6;
            case "8" -> 7;
            case "9" -> 8;
            case "10" -> 9;
            case "J" -> 10;
            case "Q" -> 11;
            case "K" -> 12;
            default -> 0;
        };
    }

    static void printGameOver() {
        System.out.println("""
                \033[1;31m\
                 /$$     /$$                        /$$                                   \s
                |  $$   /$$/                       | $$                                   \s
                 \\  $$ /$$//$$$$$$  /$$   /$$      | $$        /$$$$$$   /$$$$$$$  /$$$$$$\s
                  \\  $$$$//$$__  $$| $$  | $$      | $$       /$$__  $$ /$$_____/ /$$__  $$
                   \\  $$/| $$  \\ $$| $$  | $$      | $$      | $$  \\ $$|  $$$$$$ | $$$$$$$$
                    | $$ | $$  | $$| $$  | $$      | $$      | $$  | $$ \\____  $$| $$_____/
                    | $$ |  $$$$$$/|  $$$$$$/      | $$$$$$$$|  $$$$$$/ /$$$$$$$/|  $$$$$$$
                    |__/  \\______/  \\______/       |________/ \\______/ |_______/  \\_______/
                \033[0m""");
    }

    static void printDealerWin() {
        System.out.println("""
                \033[1;31m\
                /$$$$$$$                      /$$                           /$$      /$$ /$$
                | $$__  $$                    | $$                          | $$  /$ | $$|__/
                | $$  \\ $$  /$$$$$$   /$$$$$$ | $$  /$$$$$$   /$$$$$$       | $$ /$$$| $$ /$$ /$$$$$$$   /$$$$$$$
                | $$  | $$ /$$__  $$ |____  $$| $$ /$$__  $$ /$$__  $$      | $$/$$ $$ $$| $$| $$__  $$ /$$_____/
                | $$  | $$| $$$$$$$$  /$$$$$$$| $$| $$$$$$$$| $$  \\__/      | $$$$_  $$$$| $$| $$  \\ $$|  $$$$$$
                | $$  | $$| $$_____/ /$$__  $$| $$| $$_____/| $$            | $$$/ \\  $$$| $$| $$  | $$ \\____  $$
                | $$$$$$$/|  $$$$$$$|  $$$$$$$| $$|  $$$$$$$| $$            | $$/   \\  $$| $$| $$  | $$ /$$$$$$$/
                |_______/  \\_______/ \\_______/|__/ \\_______/|__/            |__/     \\__/|__/|__/  |__/|_______/
                \033[0m""");
    }

    static void printPlayerWin() {
        System.out.println("""
                \033[1;32m\
                 /$$$$$$$  /$$                                                             /$$                   \s
                | $$__  $$| $$                                                            |__/                   \s
                | $$  \\ $$| $$  /$$$$$$  /$$   /$$  /$$$$$$   /$$$$$$        /$$  /$$  /$$ /$$ /$$$$$$$   /$$$$$$$
                | $$$$$$$/| $$ |____  $$| $$  | $$ /$$__  $$ /$$__  $$      | $$ | $$ | $$| $$| $$__  $$ /$$_____/
                | $$____/ | $$  /$$$$$$$| $$  | $$| $$$$$$$$| $$  \\__/      | $$ | $$ | $$| $$| $$  \\ $$|  $$$$$$\s
                | $$      | $$ /$$__  $$| $$  | $$| $$_____/| $$            | $$ | $$ | $$| $$| $$  | $$ \\____  $$
                | $$      | $$|  $$$$$$$|  $$$$$$$|  $$$$$$$| $$            |  $$$$$/$$$$/| $$| $$  | $$ /$$$$$$$/
                |__/      |__/ \\_______/ \\____  $$ \\_______/|__/             \\_____/\\___/ |__/|__/  |__/|_______/\s
                                         /$$  | $$                                                               \s
                                        |  $$$$$$/                                                               \s
                                         \\______/                                                                \s
                \033[0m""");
    }

    static void delay() {
        try {
            Thread.sleep(3000);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
    }

    static void resetGame() {
        money = 10;
        gamesWon = 0;
        gamesLost = 0;
    }
}
