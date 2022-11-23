type Student = {
    id: string,
    first_names: string,
    last_name: string,
    date_of_birth: string,
}

type Score = {
    id: string,
    correct: number,
    incorrect: number,
    date: Date
}

type ScoreProps = {
    selectedStudent: Student | null,
    setStatusMessage: Function
}

enum StatusMessageLevel {Error, Warning, Info, Debug}

export { StatusMessageLevel }
export type { Student, Score, ScoreProps }
