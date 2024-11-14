# Compiler settings
CXX = g++
CXXFLAGS_DEBUG = -g -Wall -Wextra -pedantic
CXXFLAGS_OPTIMIZED = -Ofast

# Source files and output name
SRC_FILES := $(wildcard *.cpp)
OUTPUT = ChessPlusPlus
OUTPUT_DIR := C:\\Users\\benja\\Desktop\\lichess-bot-master\\engines

# Default target
all: $(OUTPUT)

# Compile the source files with debugging information
debug: $(SRC_FILES)
	@echo Compiling debugging program...
	@$(CXX) $(CXXFLAGS_DEBUG) -o $(OUTPUT)_debug $(SRC_FILES)
	@echo Done!

# Compile the source files with optimization
optimized: $(SRC_FILES)
	@echo Compiling optimized program...
	@$(CXX) $(CXXFLAGS_OPTIMIZED) -o $(OUTPUT) $(SRC_FILES)
	@echo Done!

dir: $(SRC_FILES)
	@echo Compiling optimized program for lichess engine folder...
	@$(CXX) $(CXXFLAGS_OPTIMIZED) $(SRC_FILES) -o $(OUTPUT_DIR)/$(OUTPUT)
	@echo Done!

run: optimized
	@echo Running program...
	@./$(OUTPUT)

# Clean the generated executables
clean:
	@echo Deleting executables...
	$(RM) $(OUTPUT)_debug $(OUTPUT)