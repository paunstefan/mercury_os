#include <stdio.h>
#include <ctype.h>
#include <string.h>
#include <unistd.h>
#include <stdint.h>

#define LINE_MAX 64

void help(char *);
void ls(char *);
void uname(char *);
void uptime_cmd(char *);
void run(char *);
void echo(char *);

typedef struct command
{
    char name[LINE_MAX];
    void (*exec)(char *);
} command_t;

command_t commands[] = {{.name = "help", .exec = help},
                        {.name = "ls", .exec = ls},
                        {.name = "uname", .exec = uname},
                        {.name = "uptime", .exec = uptime_cmd},
                        {.name = "run", .exec = run},
                        {.name = "echo", .exec = echo}};

typedef enum command_index
{
    HELP = 0,
    LS,
    UNAME,
    UPTIME,
    RUN,
    ECHO,
    _LAST
} command_index;

command_index read_command();
void tokenize(char *line, char **command);

void main()
{
    puts("======= MercuryOS Shell =======\n\n");

    while (1)
    {
        printf("/# ");
        char argument[LINE_MAX] = {0};
        command_index ci = read_command(argument);

        // printf("command: %d, arg: %s\n", ci, argument);
        putchar('\n');
        if (ci < _LAST)
        {

            commands[ci].exec(argument);
        }
        else
        {
            puts("Invalid command!\n");
        }
    }
}

command_index read_command(char *arg)
{
    char *command_buffer[2] = {0};

    char line_buffer[LINE_MAX] = {0};
    int i = 0;
    char c;
    while ((c = getchar()) != '\r')
    {
        // printf("%d", c);
        if (isprint(c))
        {
            putchar(c);
        }
        if (c == 127)
        {
            // Backspace
            printf("\e[1D \e[1D");
            i--;
            line_buffer[i] = 0;
            continue;
        }
        line_buffer[i] = c;
        i++;

        if (i >= LINE_MAX)
        {
            break;
        }
    }

    tokenize(line_buffer, command_buffer);

    int index = -1;
    for (int j = 0; j < _LAST; j++)
    {
        // printf("cmd: %s %s\n", command_buffer[0], commands[j].name);
        if (strcmp(command_buffer[0], commands[j].name) == 0)
        {
            index = j;
            break;
        }
    }

    if (-1 == index)
    {
        return -1;
    }

    strcpy(arg, command_buffer[1]);
    return index;
}

void tokenize(char *line, char **command)
{
    while (isspace(*line))
    {
        line = line + 1;
    }
    command[0] = line;

    while (!isspace(*line))
    {
        line = line + 1;
    }
    *line = 0;
    line = line + 1;
    while (isspace(*line))
    {
        line = line + 1;
    }
    command[1] = line;
}

void help(char *_ignore)
{
    printf("Shell usage: (command) [arguments]\n");
    printf("Commands:\n");
    printf("    - help\n");
    printf("    - ls [path]\n");
    printf("    - uname\n");
    printf("    - uptime\n");
    printf("    - run [progam path]\n");
    printf("    - echo [string]\n");
}
void ls(char *path) {}
void uname(char *_ignore)
{
    puts("MercuryOS https://github.com/paunstefan/mercury_os");
}
void uptime_cmd(char *_ignore)
{
    int64_t ut = uptime();

    printf("%ds\n", ut / 1000);
}
void run(char *path)
{
    exec(path);
}
void echo(char *string)
{
    puts(string);
}