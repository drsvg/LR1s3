#ifndef COMMAND_PARSER_H
#define COMMAND_PARSER_H

#include "common.h"

struct ParsedCommand {
    string raw;                  // исходная строка запроса
    vector<string> tokens;       // разбитые токены
    ContainerType type;          // тип контейнера по первой букве команды
    string command;              // полное имя команды (например, "MPUSH")
};

// Разбор строки запроса
ParsedCommand parseCommand(const string& query);

// Определение типа контейнера по первой букве команды
ContainerType containerTypeFromCommand(const string& command);

// Утилита: разбить строку по пробелам
vector<string> splitTokens(const string& s);

#endif