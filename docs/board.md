# Structure of Tart Boards

- each board should be its own yaml file
- filenames must be unique, names are the id
- different board types...?
- archive feature?
- indexing the tasks!

## board
  - name: String = `"board-name"`
  - collaborators: String[] = `"collaborator-name <email>"`
  - stages: (ordered by progression, of course)
    - todo: Task[]
    - doing: Task[]
    - {any other task categories}: Task[]
    - done: Task[]
  - tags: String[]

## Task
  - id: String = `hash`
    - generated at creation using author, board, title, etc. for unique hash
  - index: u32 = easier way to reference tasks
    - start at 0
  - title: String
  - description: String
  - collaborators?
  - time_created:
  - deadline:
  - tags: String[]
  - etc.
